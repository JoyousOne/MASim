use std::collections::HashMap;

use macroquad::{
    color::Color,
    math::{IVec2, Vec2},
};

use crate::{
    agent::{
        learning_agent::{Action, Done},
        state::Value,
    },
    interface::grid::{Grid, GridSize},
    scheduler::scheduler::{AgentRef, Position},
};

pub struct Env {
    grid: Grid,
    pub actions: Vec<Action>,

    /// Element with persistent long term position such as obstacles (walls, bushes, etc.), the goal cell, etc.
    /// Unlike the grid, those are in an hashmap because if an agent need to check a cell we want to have an access of O(1)
    pub persistent_elements: HashMap<Position, Color>,
    pub data: HashMap<u32, Value>,
}

impl Env {
    pub fn new(
        start: Vec2,
        end: Vec2,
        size: GridSize,
        persistent_elements: HashMap<Position, Color>,
        actions: &[Action],
        data: HashMap<u32, Value>,
    ) -> Env {
        let grid_persistent_elements: Vec<(Position, Color)> = persistent_elements
            .iter()
            .map(|(&pos, &color)| (pos, color))
            .collect();

        Env {
            grid: Grid::new(start, end, size, grid_persistent_elements),
            actions: Vec::from(actions),
            persistent_elements: persistent_elements,
            data,
        }
    }

    pub fn get_width(&self) -> &usize {
        &self.grid.size.width
    }

    pub fn get_heigth(&self) -> &usize {
        &self.grid.size.heigth
    }

    pub fn display_grid(
        &mut self,
        start: Vec2,
        end: Vec2,
        grid_color: Color,
        agent_positions: Vec<(IVec2, Color)>,
    ) {
        self.grid.display(start, end, grid_color, agent_positions);
    }

    pub fn valid_position(&self, position: IVec2) -> bool {
        let IVec2 { x, y } = position;

        x >= 0 && x < self.grid.size.width as i32 && // x
        y >= 0 && y < self.grid.size.heigth as i32 // y
    }

    pub fn step(&mut self, position: Position, agent: &mut AgentRef) -> (Position, Done) {
        let mut agent = agent.borrow_mut();

        let action = agent.choose_action(&agent.state, &self.actions);

        let (new_position, next_state, reward, done) =
            agent.step(self, position, &agent.state, &action);

        let state = agent.state.clone();

        // Update the agent state
        agent.update(
            &state,
            &action,
            reward,
            &next_state,
            &self.actions, // THIS SHOULD POSSIBLY VARY
        );

        agent.update_state(next_state);

        (new_position, done)
    }

    pub fn get_random_position(&self) -> Position {
        Position {
            x: rand::random_range(0..*self.get_width() as i32),
            y: rand::random_range(0..*self.get_heigth() as i32),
        }
    }

    // TODO implement a way to update persistent_element
    //pub fn update_persistent_element(&mut self, position: Position, color: Color)

    pub fn move_persistent_element(&mut self, current_position: Position, new_position: Position) {
        if let Some(element) = self.persistent_elements.remove(&current_position) {
            self.persistent_elements.insert(new_position, element);

            let grid_persistent_elements: Vec<(Position, Color)> = self
                .persistent_elements
                .iter()
                .map(|(&pos, &color)| (pos, color))
                .collect();

            self.grid
                .update_persistent_element(grid_persistent_elements);
        }
    }
}

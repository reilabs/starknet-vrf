use dojo_starter::models::moves::Direction;
use dojo_starter::models::position::Position;

// define the interface
#[dojo::interface]
trait IActions {
    fn spawn();
    fn move(direction: Direction);
}

// dojo decorator
#[dojo::contract]
mod actions {
    use super::{IActions, next_position};
    use starknet::{ContractAddress, get_caller_address};
    use starknet::testing::cheatcode;
    use dojo_starter::models::{position::{Position, Vec2}, moves::{Moves, Direction}};
    use hint_vrf::ecvrf::{Point, Proof, ECVRFImpl};

    #[abi(embed_v0)]
    impl ActionsImpl of IActions<ContractState> {
        fn spawn(world: IWorldDispatcher) {
            let pk = Point {
                x: 116790107469130620194501433118398966236215846997329127478236149064647078075,
                y: 924164433250424617580268877992838433046851481016864292109466731590665978751
            };
            let mut seed = ArrayTrait::new();
    
            let mut result: Span<felt252> = cheatcode::<'vrf'>(seed.span());
            let proof: Proof = Serde::deserialize(ref result).unwrap();
            let ecvrf = ECVRFImpl::new(pk);
            ecvrf.verify(proof, seed.span()).unwrap();
        }

        // Implementation of the move function for the ContractState struct.
        fn move(world: IWorldDispatcher, direction: Direction) {
            // Get the address of the current caller, possibly the player's address.
            let player = get_caller_address();

            // Retrieve the player's current position and moves data from the world.
            let (mut position, mut moves) = get!(world, player, (Position, Moves));

            // Deduct one from the player's remaining moves.
            moves.remaining -= 1;

            // Update the last direction the player moved in.
            moves.last_direction = direction;

            // Calculate the player's next position based on the provided direction.
            let next = next_position(position, direction);

            // Update the world state with the new moves data and position.
            set!(world, (moves, next));
            // Emit an event to the world to notify about the player's move.
            emit!(world, (moves));
        }
    }
}

// Define function like this:
fn next_position(mut position: Position, direction: Direction) -> Position {
    match direction {
        Direction::None => { return position; },
        Direction::Left => { position.vec.x -= 1; },
        Direction::Right => { position.vec.x += 1; },
        Direction::Up => { position.vec.y -= 1; },
        Direction::Down => { position.vec.y += 1; },
    };
    position
}

#[cfg(test)]
mod tests {
    use starknet::class_hash::Felt252TryIntoClassHash;
    use starknet::testing::cheatcode;
    
    // import ECVRF
    use hint_vrf::ecvrf::{Point, Proof, ECVRFImpl};

    // import world dispatcher
    use dojo::world::{IWorldDispatcher, IWorldDispatcherTrait};
    // import test utils
    use dojo::test_utils::{spawn_test_world, deploy_contract};
    // import test utils
    use dojo_starter::{
        systems::{actions::{actions, IActionsDispatcher, IActionsDispatcherTrait}},
        models::{position::{Position, Vec2, position}, moves::{Moves, Direction, moves}}
    };

    #[test]
    #[available_gas(500000000)]
    fn test_cheatcode() {
        let pk = Point {
            x: 116790107469130620194501433118398966236215846997329127478236149064647078075,
            y: 924164433250424617580268877992838433046851481016864292109466731590665978751
        }; 

        println!("Hello world!");
        let mut seed = ArrayTrait::new();
        let mut result = cheatcode::<'vrf'>(seed.span());
        let proof: Proof = Serde::deserialize(ref result).unwrap();
        println!("Result is {}", proof.gamma.x);

        let ecvrf = ECVRFImpl::new(pk);
        let result = ecvrf.verify(proof, seed.span());
        println!("Dice roll {result:?}");

    }

    fn proof_from_oracle() -> Proof {
        Proof {
            gamma: Point {
                x: 1506339363762384048749124975867331702319430609263271304275332020910807468800,
                y: 36259598506905210600179635686591002688831785399437338349196739602416217657
            },
            c: 2613903846701008054856365693011070443633034612733309583190565217827378733393,
            s: 1867682679224997956048283066055885717352683300581532690215097247223135564277,
        }
    }

    // #[test]
    // #[available_gas(30000000)]
    // fn test_move() {
    //     // caller
    //     let caller = starknet::contract_address_const::<0x0>();

    //     // models
    //     let mut models = array![position::TEST_CLASS_HASH, moves::TEST_CLASS_HASH];

    //     // deploy world with models
    //     let world = spawn_test_world(models);

    //     // deploy systems contract
    //     let contract_address = world
    //         .deploy_contract('salt', actions::TEST_CLASS_HASH.try_into().unwrap());
    //     let actions_system = IActionsDispatcher { contract_address };

    //     // call spawn()
    //     actions_system.spawn();

    //     // call move with direction right
    //     actions_system.move(Direction::Right);

    //     // Check world state
    //     let moves = get!(world, caller, Moves);

    //     // casting right direction
    //     let right_dir_felt: felt252 = Direction::Right.into();

    //     // check moves
    //     assert(moves.remaining == 99, 'moves is wrong');

    //     // check last direction
    //     assert(moves.last_direction.into() == right_dir_felt, 'last direction is wrong');

    //     // get new_position
    //     let new_position = get!(world, caller, Position);

    //     // check new position x
    //     assert(new_position.vec.x == 11, 'position x is wrong');

    //     // check new position y
    //     assert(new_position.vec.y == 10, 'position y is wrong');
    // }
}

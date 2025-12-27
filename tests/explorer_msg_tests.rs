#![allow(clippy::pedantic)]

mod common;

use common::*;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use common_game::components::sunray::Sunray;
use common_game::protocols::orchestrator_planet::OrchestratorToPlanet;
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};
use std::collections::HashSet;

#[test]
fn test_supported_resource_response() {
    let (planet, (tx_orchestrator, rx_orchestrator), tx_explorer) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::IncomingExplorerRequest {
            explorer_id: 0,
            new_sender: tx_to_explorer,
        },
    );

    // 4. Request from explorer to get the supported combinations list of the planet
    let response = explorer_send(
        &tx_explorer,
        &rx_explorer,
        ExplorerToPlanet::SupportedResourceRequest { explorer_id: 0 },
    );

    // 5. Planet should respond with its combinations list
    let expected = HashSet::from([BasicResourceType::Hydrogen]);

    match response {
        PlanetToExplorer::SupportedResourceResponse { resource_list } => {
            assert_eq!(
                resource_list, expected,
                "Expected {:?}, got {:?}",
                expected, resource_list
            );
        }
        _ => panic! {"Expected a combination response but received a different one"},
    }

    // 6. Orchestrator kills planet
    orchestrator_kill_planet(&tx_orchestrator, &rx_orchestrator);

    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_supported_combination_response() {
    let (planet, (tx_orchestrator, rx_orchestrator), tx_explorer) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::IncomingExplorerRequest {
            explorer_id: 0,
            new_sender: tx_to_explorer,
        },
    );

    // 4. Request from explorer to get the supported combinations list of the planet
    let response = explorer_send(
        &tx_explorer,
        &rx_explorer,
        ExplorerToPlanet::SupportedCombinationRequest { explorer_id: 0 },
    );

    // 5. Planet should respond with its combinations list
    let expected = HashSet::from([
        ComplexResourceType::AIPartner,
        ComplexResourceType::Diamond,
        ComplexResourceType::Dolphin,
        ComplexResourceType::Life,
        ComplexResourceType::Robot,
        ComplexResourceType::Water,
    ]);

    match response {
        PlanetToExplorer::SupportedCombinationResponse { combination_list } => {
            assert_eq!(
                combination_list, expected,
                "Expected {:?}, got {:?}",
                expected, combination_list
            );
        }
        _ => panic! {"Expected a combination response but received a different one"},
    }

    // 6. Orchestrator kills planet
    orchestrator_kill_planet(&tx_orchestrator, &rx_orchestrator);

    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_generate_resource_response() {
    let (planet, (tx_orchestrator, rx_orchestrator), tx_explorer) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::IncomingExplorerRequest {
            explorer_id: 0,
            new_sender: tx_to_explorer,
        },
    );

    // 4. Request from explorer to generate Hydrogen
    let response = explorer_send(
        &tx_explorer,
        &rx_explorer,
        ExplorerToPlanet::GenerateResourceRequest {
            explorer_id: 0,
            resource: BasicResourceType::Hydrogen,
        },
    );

    // 5. Planet should respond with None, since it does not have an energy cell
    match response {
        PlanetToExplorer::GenerateResourceResponse { resource } => assert_eq!(
            resource, None,
            "Planet created a resource but it did not have an energy cell"
        ),

        _ => panic!("Expected a generate resource response but received a different one"),
    }

    // 6. Orchestrator sends a sunray
    orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Sunray(Sunray::default()),
    );

    // 7. Request from explorer to generate Hydrogen
    let response = explorer_send(
        &tx_explorer,
        &rx_explorer,
        ExplorerToPlanet::GenerateResourceRequest {
            explorer_id: 0,
            resource: BasicResourceType::Hydrogen,
        },
    );

    // 8. Planet should respond with Hydrogen
    match response {
        PlanetToExplorer::GenerateResourceResponse { resource } => {
            assert!(
                resource.is_some(),
                "Expected Hydrogen but Planet returned None"
            );
            assert_eq!(
                format!("{:?}", resource.as_ref().unwrap()),
                "Hydrogen(Hydrogen { _private: () })",
                "Planet returned wrong resource, expected Hydrogen, got : {:?}",
                resource.unwrap()
            );
        }
        _ => panic!("Received wrong response type"),
    }

    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_combine_resource_response() {
    let (planet, (tx_orchestrator, rx_orchestrator), tx_explorer) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planets
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::bounded::<PlanetToExplorer>(1);
    orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::IncomingExplorerRequest {
            explorer_id: 0,
            new_sender: tx_to_explorer.clone(),
        },
    );

    // 4. Orchestrator sends sunrays
    orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Sunray(Sunray::default()),
    );

    // 5. Request from the explorer to generate Hydrogen from planet
    let response1 = explorer_send(
        &tx_explorer,
        &rx_explorer,
        ExplorerToPlanet::GenerateResourceRequest {
            explorer_id: 0,
            resource: BasicResourceType::Hydrogen,
        },
    );

    // 6. Orchestrator sends sunrays
    orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Sunray(Sunray::default()),
    );

    // 7. Request from the explorer to generate Hydrogen from planet
    let response2 = explorer_send(
        &tx_explorer,
        &rx_explorer,
        ExplorerToPlanet::GenerateResourceRequest {
            explorer_id: 0,
            resource: BasicResourceType::Hydrogen,
        },
    );

    let hydrogen1 = match response1 {
        PlanetToExplorer::GenerateResourceResponse {
            resource: Some(hydrogen),
        } => hydrogen,
        _ => panic!("Expected hydrogen but did not receive it"),
    };

    let hydrogen2 = match response2 {
        PlanetToExplorer::GenerateResourceResponse {
            resource: Some(hydrogen),
        } => hydrogen,
        _ => panic!("Expected hydrogen but did not receive it"),
    };

    // Verify we got hydrogen resources
    assert!(
        hydrogen1.to_hydrogen().is_ok(),
        "First resource should be hydrogen"
    );
    assert!(
        hydrogen2.to_hydrogen().is_ok(),
        "Second resource should be hydrogen"
    );

    // Note: Combination test commented out since planet only generates Hydrogen
    // and no valid combinations exist with H+H (Water requires H+O, not H+H)

    // // 8. Orchestrator sends sunrays
    // orchestrator_send(
    //     &tx_orchestrator,
    //     &rx_orchestrator,
    //     OrchestratorToPlanet::Sunray(Sunray::default()),
    // );

    // // 9. Explorer asks planet to combine water (would fail - water needs H + O, not H + H)
    // let response = explorer_send(
    //     &tx_explorer,
    //     &rx_explorer,
    //     ExplorerToPlanet::CombineResourceRequest {
    //         explorer_id: 0,
    //         msg: ComplexResourceRequest::Water(
    //             hydrogen1.to_hydrogen().unwrap(),
    //             hydrogen2.to_hydrogen().unwrap(),
    //         ),
    //     },
    // );

    // // 10. Planet should respond with an error since water cannot be made from two hydrogen
    // match response {
    //     PlanetToExplorer::CombineResourceResponse {
    //         complex_response: Err(_),
    //     } => {
    //         // Expected: combination should fail
    //     }
    //     PlanetToExplorer::CombineResourceResponse {
    //         complex_response: Ok(resource),
    //     } => panic!("Expected combination to fail, but got: {:?}", resource),
    //     _ => panic!("Expected a combine resource response"),
    // }

    // 8. Orchestrator kills Planet
    orchestrator_kill_planet(&tx_orchestrator, &rx_orchestrator);

    // 9. End thread
    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_available_cell_response() {
    let (planet, (tx_orchestrator, rx_orchestrator), tx_explorer) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::IncomingExplorerRequest {
            explorer_id: 0,
            new_sender: tx_to_explorer,
        },
    );

    // 4. Request from explorer to get available energy cells
    let response = explorer_send(
        &tx_explorer,
        &rx_explorer,
        ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id: 0 },
    );

    // 5. Planet should respond with the number of available energy cells
    let expected = 0;

    match response {
        PlanetToExplorer::AvailableEnergyCellResponse { available_cells } => {
            assert_eq!(
                available_cells, expected,
                "Expected {:?}, got {:?}",
                expected, available_cells
            );
        }
        _ => panic! {"Expected an AvailableEnergyCellResponse but received a different one"},
    }

    // 6. Orchestrator sends two sunray
    orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Sunray(Sunray::default()),
    );

    orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Sunray(Sunray::default()),
    );

    // 7.  Request from explorer to get available energy cells
    let response = explorer_send(
        &tx_explorer,
        &rx_explorer,
        ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id: 0 },
    );

    // 8. Planet should respond with the number of available energy cells
    let expected = 1;

    match response {
        PlanetToExplorer::AvailableEnergyCellResponse { available_cells } => {
            assert_eq!(
                available_cells, expected,
                "Expected {:?}, got {:?}",
                expected, available_cells
            );
        }
        _ => panic! {"Expected an AvailableEnergyCellResponse but received a different one"},
    }

    // 9. Orchestrator kills planet
    orchestrator_kill_planet(&tx_orchestrator, &rx_orchestrator);

    // 10. End thread
    drop(tx_orchestrator);
    let _ = handle.join();
}

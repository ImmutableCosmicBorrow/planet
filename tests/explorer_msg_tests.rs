mod common;

use std::collections::HashSet;
use crossbeam_channel;
use common_game::components::resource::{BasicResourceType, ComplexResourceRequest, ComplexResourceType};
use common_game::components::sunray::Sunray;
use common_game::protocols::messages::{ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer};
use common::*;

#[test]
fn test_supported_resource_response(){
    let (planet,
        (tx_orchestrator, rx_orchestrator),
        tx_explorer) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer});


    // 4. Request from explorer to get the supported combinations list of the planet
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::SupportedResourceRequest {explorer_id : 0});

    // 5. Planet should respond with its combinations list
    let expected = HashSet::from([BasicResourceType::Carbon]);

    match response {
        PlanetToExplorer::SupportedResourceResponse {resource_list} => {
            assert_eq!(resource_list, expected, "Expected {:?}, got {:?}", expected, resource_list);
        },
        _ => panic!{"Expected a combination response but received a different one"},
    }

    // 6. Orchestrator stops planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);


    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_supported_combination_response() {
    let (planet,
        (tx_orchestrator, rx_orchestrator),
        tx_explorer) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer});


    // 4. Request from explorer to get the supported combinations list of the planet
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::SupportedCombinationRequest {explorer_id : 0});

    // 5. Planet should respond with its combinations list
    let expected = HashSet::from([ComplexResourceType::AIPartner, ComplexResourceType::Diamond,
        ComplexResourceType::Dolphin, ComplexResourceType::Life, ComplexResourceType::Robot,
        ComplexResourceType::Water]);

    match response {
        PlanetToExplorer::SupportedCombinationResponse {combination_list} => {
            assert_eq!(combination_list, expected, "Expected {:?}, got {:?}", expected, combination_list);
        },
        _ => panic!{"Expected a combination response but received a different one"},
    }

    // 6. Orchestrator stops planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);


    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_generate_resource_response(){

    let (planet,
        (tx_orchestrator, rx_orchestrator),
        tx_explorer) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer});

    // 4. Request from explorer to generate Oxygen
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::GenerateResourceRequest {explorer_id : 0, resource : BasicResourceType::Carbon});

    // 5. Planet should respond with None, since it does not have an energy cell
    match response {
        PlanetToExplorer::GenerateResourceResponse {resource} => assert_eq!(resource, None, "Planet created a resource but it did not have an energy cell"),

        _ => panic!("Expected a generate resource response but received a different one"),
    }

    // 6. Orchestrator sends a sunray
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));

    // 7. Request from explorer to generate Carbon
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::GenerateResourceRequest {
        explorer_id: 0,
        resource: BasicResourceType::Carbon
    });


    // 8. Planet should respond with Carbon
    match response {
        PlanetToExplorer::GenerateResourceResponse {resource} => {
            assert!(resource.is_some(), "Expected Carbon but Planet returned None");
            assert_eq!(format!("{:?}", resource.as_ref().unwrap()), "Carbon(Carbon { _private: () })", "Planet returned wrong resource, expected Carbon, got : {:?}", resource.unwrap());
        }
        _ => panic!("Received wrong response type")
    }


    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_combine_resource_response(){

    let (planet,
        (tx_orchestrator, rx_orchestrator),
        tx_explorer) = create_test_planet();


    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planets
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);




    // 3. Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::bounded::<PlanetToExplorer>(1);
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer.clone()});

    // 4. Orchestrator sends sunrays
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));


    // 5. Request from the explorer to generate Carbon from planet
    let response1 = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::GenerateResourceRequest { explorer_id: 0, resource: BasicResourceType::Carbon});

    // 6. Orchestrator sends sunrays
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));


    // 7. Request from the explorer to generate Carbon from planet
    let response2 = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::GenerateResourceRequest { explorer_id: 0, resource: BasicResourceType::Carbon});

    let carbon1 = match response1 {
        PlanetToExplorer::GenerateResourceResponse {resource : Some(carbon)} => { carbon },
        _ => panic!("Expected carbon but did not receive it"),
    };

    let carbon2 = match response2 {
        PlanetToExplorer::GenerateResourceResponse {resource : Some(carbon)} => { carbon },
        _ => panic!("Expected oxygen but did not receive it"),
    };

    // 8. Orchestrator sends sunrays
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));


    // 9. Explorer asks planet to combine diamond
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::CombineResourceRequest { explorer_id: 0, msg: ComplexResourceRequest::Diamond(carbon1.to_carbon().unwrap(), carbon2.to_carbon().unwrap() ) });

    // 10. Planet should respond with Diamond
    match response {
        PlanetToExplorer::CombineResourceResponse { complex_response : Ok(diamond) } => assert_eq!(format!{"{:?}", diamond}, "Diamond(Diamond { _private: () })", "{}", format!{"Expected Diamond, got: {:?}", diamond}),
        _ => panic!("Expected a diamond but did not receive it"),
    }

    // 11. Orchestrator stops Planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);

    // 12. End thread
    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_available_cell_response(){
    let (planet,
        (tx_orchestrator, rx_orchestrator),
        tx_explorer) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer});


    // 4. Request from explorer to get available energy cells
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::AvailableEnergyCellRequest {explorer_id : 0});

    // 5. Planet should respond with the number of available energy cells
    let expected = 0;

    match response {
        PlanetToExplorer::AvailableEnergyCellResponse {available_cells} => {
            assert_eq!(available_cells, expected, "Expected {:?}, got {:?}", expected, available_cells);
        },
        _ => panic!{"Expected an AvailableEnergyCellResponse but received a different one"},
    }

    // 6. Orchestrator sends sunray
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));


    // 7.  Request from explorer to get available energy cells
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::AvailableEnergyCellRequest {explorer_id : 0});

    // 8. Planet should respond with the number of available energy cells
    let expected = 1;

    match response {
        PlanetToExplorer::AvailableEnergyCellResponse {available_cells} => {
            assert_eq!(available_cells, expected, "Expected {:?}, got {:?}", expected, available_cells);
        },
        _ => panic!{"Expected an AvailableEnergyCellResponse but received a different one"},
    }

    // 9. Orchestrator stops planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);

    // 10. End thread
    drop(tx_orchestrator);
    let _ = handle.join();
}
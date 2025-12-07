mod utils;

use std::collections::HashSet;
use common_game::components::asteroid::Asteroid;
use crossbeam_channel;
use common_game::components::resource::{BasicResourceType, Carbon, ComplexResourceRequest, ComplexResourceType, Water};
use common_game::components::sunray::Sunray;
use common_game::protocols::messages::{ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator};
use planet::{create_planet, Ai};
use utils::*;

#[test]
fn test_supported_resource_response(){
    let (planet,
        (tx_orchestrator, rx_orchestrator),
        tx_explorer) = create_test_planet();

    // start thread
    let handle = start_thread(planet);

    // Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer});


    // Request from explorer to get the supported combinations list of the planet
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::SupportedResourceRequest {explorer_id : 0});

    // Planet should respond with its combinations list
    let expected = HashSet::from([BasicResourceType::Carbon]);

    match response {
        PlanetToExplorer::SupportedResourceResponse {resource_list} => {
            assert_eq!(resource_list, expected, "Expected {:?}, got {:?}", expected, resource_list);
        },
        _ => panic!{"Expected a combination response but received a different one"},
    }

    // Orchestrator stops planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);


    drop(tx_orchestrator);
    drop(tx_explorer);
    let _ = handle.join();
}
#[test]
fn test_supported_combination_response() {
    let (planet,
        (tx_orchestrator, rx_orchestrator),
        tx_explorer) = create_test_planet();

    // start thread
    let handle = start_thread(planet);

    // Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer});


    // Request from explorer to get the supported combinations list of the planet
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::SupportedCombinationRequest {explorer_id : 0});

    // Planet should respond with its combinations list
    let expected = HashSet::from([ComplexResourceType::AIPartner, ComplexResourceType::Diamond,
        ComplexResourceType::Dolphin, ComplexResourceType::Life, ComplexResourceType::Robot,
        ComplexResourceType::Water]);

    match response {
        PlanetToExplorer::SupportedCombinationResponse {combination_list} => {
            assert_eq!(combination_list, expected, "Expected {:?}, got {:?}", expected, combination_list);
        },
        _ => panic!{"Expected a combination response but received a different one"},
    }

    // Orchestrator stops planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);


    drop(tx_orchestrator);
    drop(tx_explorer);
    let _ = handle.join();
}
#[test]
fn test_generate_resource_response(){
    // TODO: behaviour might change when planet is not deterministic

    let (planet,
        (tx_orchestrator, rx_orchestrator),
        tx_explorer) = create_test_planet();

    let handle = start_thread(planet);

    // Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer});

    // Request from explorer to generate Oxygen
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::GenerateResourceRequest {explorer_id : 0, resource : BasicResourceType::Carbon});

    // Planet should respond with None, since it does not have an energy cell
    match response {
        PlanetToExplorer::GenerateResourceResponse {resource} => assert_eq!(resource, None, "Planet created a resource but it did not have an energy cell"),

        _ => panic!("Expected a generate resource response but received a different one"),
    }

    // Orchestrator sends a sunray
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));

    // Request from explorer to generate Carbon
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::GenerateResourceRequest {
        explorer_id: 0,
        resource: BasicResourceType::Carbon
    });


    // Planet should respond with Carbon
    match response {
        PlanetToExplorer::GenerateResourceResponse {resource} => {
            assert!(resource.is_some(), "Expected Carbon but Planet returned None");
            assert_eq!(format!("{:?}", resource.as_ref().unwrap()), "Carbon(Carbon { _private: () })", "Planet returned wrong resource, expected Carbon, got : {:?}", resource.unwrap());
        }
        _ => panic!("Received wrong response type")
    }


    drop(tx_orchestrator);
    drop(tx_explorer);
    let _ = handle.join();
}
#[test]
fn test_combine_resource_response(){
    //TODO: since planet C can generate only 1 BasicResourceType idk how we can test the combine request
    // because we would need 2 different basic resources in order to send a CombineResourceRequest, and we cannot construct them since they have a private field
    // we could create 2 different type C planets and thus generate 2 different basic resources, then combine, but it's a bit complex


    let (planet,
        (tx_orchestrator, rx_orchestrator),
        tx_explorer) = create_test_planet();



    let handle = start_thread(planet);

    // Orchestrator starts the planets
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);




    // Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::bounded::<PlanetToExplorer>(1);
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer.clone()});

    // Orchestrator sends sunrays
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));


    // Request from the explorer to generate Carbon from planet1
    let response1 = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::GenerateResourceRequest { explorer_id: 0, resource: BasicResourceType::Carbon});

    // Orchestrator sends sunrays
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));


    // Request from the explorer to generate Carbon from planet1
    let response2 = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::GenerateResourceRequest { explorer_id: 0, resource: BasicResourceType::Carbon});



    let carbon1 = match response1 {
        PlanetToExplorer::GenerateResourceResponse {resource : Some(carbon)} => { carbon },
        _ => panic!("Expected carbon but did not receive it"),
    };
    let carbon2 = match response2 {
        PlanetToExplorer::GenerateResourceResponse {resource : Some(carbon)} => { carbon },
        _ => panic!("Expected oxygen but did not receive it"),
    };

    // Orchestrator sends sunrays
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));


    // Explorer asks planet1 to combine diamond
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::CombineResourceRequest { explorer_id: 0, msg: ComplexResourceRequest::Diamond(carbon1.to_carbon().unwrap(), carbon2.to_carbon().unwrap() ) });

    // Planet should respond with Diamond
    match response {
        PlanetToExplorer::CombineResourceResponse { complex_response : Ok(diamond) } => assert_eq!(format!{"{:?}", diamond}, "Diamond(Diamond { _private: () })", "{}", format!{"Expected Diamond, got: {:?}", diamond}),
        _ => panic!("Expected a diamond but did not receive it"),
    }




    // Request from explorer to combine Water
    //let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::CombineResourceRequest {explorer_id : 0, msg : ComplexResourceRequest::Water(Hydrogen{_private : ()}, Oxygen{_private : ()})});

    // Planet should respond with None, since it does not have an energy cell

    // Orchestrator sends a sunray
    //orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));

    // Request from explorer to combine Water

    // Planet should respond with Water

    drop(tx_orchestrator);
    drop(tx_explorer);

    let _ = handle.join();
}
#[test]
fn test_available_cell_response(){
    // TODO: when planet choice making is implemented, planet might choose to behave differently from what i expected here (when it was deterministic), so this might need to change
    
    let (planet,
        (tx_orchestrator, rx_orchestrator),
        tx_explorer) = create_test_planet();

    // start thread
    let handle = start_thread(planet);

    // Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // Orchestrator tells the planet that an explorer arrived
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::unbounded::<PlanetToExplorer>();
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer});


    // Request from explorer to get available energy cells
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::AvailableEnergyCellRequest {explorer_id : 0});

    // Planet should respond with the number of available energy cells
    let expected = 0;

    match response {
        PlanetToExplorer::AvailableEnergyCellResponse {available_cells} => {
            assert_eq!(available_cells, expected, "Expected {:?}, got {:?}", expected, available_cells);
        },
        _ => panic!{"Expected an AvailableEnergyCellResponse but received a different one"},
    }

    // Orchestrator sends sunray
    orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));


    // Request from explorer to get available energy cells
    let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::AvailableEnergyCellRequest {explorer_id : 0});

    // Planet should respond with the number of available energy cells
    let expected = 1;

    match response {
        PlanetToExplorer::AvailableEnergyCellResponse {available_cells} => {
            assert_eq!(available_cells, expected, "Expected {:?}, got {:?}", expected, available_cells);
        },
        _ => panic!{"Expected an AvailableEnergyCellResponse but received a different one"},
    }

    // Orchestrator stops planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);


    drop(tx_orchestrator);
    drop(tx_explorer);
    let _ = handle.join();
}
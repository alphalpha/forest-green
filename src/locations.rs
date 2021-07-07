use crate::util;
use std::collections::HashMap;

type LocationMap = HashMap<String, String>;
pub fn location_map() -> Result<LocationMap, util::Error> {
    let mut location_map: LocationMap = HashMap::new();
    location_map.insert("MC100".to_string(), "Tammela, canopy".to_string());
    location_map.insert("MC101".to_string(), "Tammela, ground".to_string());
    location_map.insert("MC102".to_string(), "Tammela, crown".to_string());

    location_map.insert("MC103".to_string(), "Punkaharju, ground".to_string());
    location_map.insert("MC104".to_string(), "Punkaharju, crown".to_string());
    location_map.insert("MC105".to_string(), "Punkaharju, landscape".to_string());

    location_map.insert("MC106".to_string(), "Hyytiälä, crown".to_string());
    location_map.insert("MC107".to_string(), "Hyytiälä, ground".to_string());

    location_map.insert("MC100".to_string(), "Tammela, canopy".to_string());
    location_map.insert("MC101".to_string(), "Tammela, ground".to_string());

    location_map.insert("MC108".to_string(), "Sodankylä, forest, canopy".to_string());
    location_map.insert("MC109".to_string(), "Sodankylä, forest, crown".to_string());
    location_map.insert("MC110".to_string(), "Sodankylä, forest, ground".to_string());
    location_map.insert(
        "MC111".to_string(),
        "Sodankylä, wetland, ground".to_string(),
    );

    location_map.insert("MC112".to_string(), "Parkano, landscape".to_string());

    location_map.insert("MC113".to_string(), "Suonenjoki, canopy".to_string());

    location_map.insert("MC114".to_string(), "Kenttärova, canopy".to_string());
    location_map.insert("MC115".to_string(), "Kenttärova, crown".to_string());
    location_map.insert("MC116".to_string(), "Kenttärova, ground".to_string());

    location_map.insert("MC117".to_string(), "Paljakka, landscape".to_string());
    location_map.insert("MC118".to_string(), "Paljakka, landscape".to_string());
    location_map.insert("MC117-1".to_string(), "Paljakka, landscape".to_string());

    location_map.insert("MC119".to_string(), "Värriö, canopy".to_string());
    location_map.insert("MC120".to_string(), "Värriö, crown".to_string());
    location_map.insert("MC121".to_string(), "Värriö, ground".to_string());

    location_map.insert("MC122".to_string(), "Lammi, crown".to_string());
    location_map.insert("MC123".to_string(), "Lammi, crown".to_string());
    location_map.insert("MC124".to_string(), "Lammi, landscape".to_string());
    location_map.insert("MC125".to_string(), "Lammi, landscape".to_string());
    location_map.insert("MC126".to_string(), "Lammi, ground".to_string());
    location_map.insert("MC127".to_string(), "Lammi, ground".to_string());

    location_map.insert("MC128".to_string(), "Kaamanen, ground".to_string());

    location_map.insert("MC129".to_string(), "Lompolojänkkä, ground".to_string());

    location_map.insert("MC130".to_string(), "Tvärminne, landscape".to_string());

    location_map.insert("MC131".to_string(), "Jokioinen, landscape".to_string());

    Ok(location_map)
}

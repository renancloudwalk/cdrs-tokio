mod common;

#[cfg(feature = "e2e-tests")]
use common::*;

#[cfg(feature = "e2e-tests")]
use cdrs_tokio::query_values;
#[cfg(feature = "e2e-tests")]
use cdrs_tokio::types::blob::Blob;
#[cfg(feature = "e2e-tests")]
use cdrs_tokio::types::list::List;
#[cfg(feature = "e2e-tests")]
use cdrs_tokio::types::map::Map;
#[cfg(feature = "e2e-tests")]
use cdrs_tokio::types::AsRust;
#[cfg(feature = "e2e-tests")]
use cdrs_tokio::types::ByName;
#[cfg(feature = "e2e-tests")]
use maplit::hashmap;
#[cfg(feature = "e2e-tests")]
use uuid::Uuid;

#[cfg(feature = "e2e-tests")]
use std::collections::HashMap;
#[cfg(feature = "e2e-tests")]
use std::str::FromStr;

#[tokio::test]
#[cfg(feature = "e2e-tests")]
async fn list() {
    let cql = "CREATE TABLE IF NOT EXISTS cdrs_test.test_lists \
               (my_text_list frozen<list<text>> PRIMARY KEY, \
               my_nested_list list<frozen<list<int>>>)";
    let session = setup(cql).await.expect("setup");

    let my_text_list = vec!["text1", "text2", "text3"];
    let my_nested_list: Vec<Vec<i32>> =
        vec![vec![1, 2, 3], vec![999, 888, 777, 666, 555], vec![-1, -2]];
    let values = query_values!(my_text_list.clone(), my_nested_list.clone());

    let cql = "INSERT INTO cdrs_test.test_lists \
               (my_text_list, my_nested_list) VALUES (?, ?)";
    session
        .query_with_values(cql, values)
        .await
        .expect("insert lists error");

    let cql = "SELECT * FROM cdrs_test.test_lists";
    let rows = session
        .query(cql)
        .await
        .expect("query lists error")
        .response_body()
        .expect("get body with lists error")
        .into_rows()
        .expect("converting body with lists into rows error");

    assert_eq!(rows.len(), 1);
    for row in rows {
        let my_text_list_row: Vec<String> = row
            .r_by_name::<List>("my_text_list")
            .expect("my_text_list")
            .as_r_rust()
            .expect("my_text_list as rust");
        let my_nested_list_outer_row: Vec<List> = row
            .r_by_name::<List>("my_nested_list")
            .expect("my_nested_list")
            .as_r_rust()
            .expect("my_nested_list (outer) as rust");
        let mut my_nested_list_row = Vec::with_capacity(my_nested_list_outer_row.len());
        for my_nested_list_inner_row in my_nested_list_outer_row {
            let my_nested_list_inner_row: Vec<i32> = my_nested_list_inner_row
                .as_r_rust()
                .expect("my_nested_list (inner) as rust");
            my_nested_list_row.push(my_nested_list_inner_row);
        }
        assert_eq!(my_text_list_row, vec!["text1", "text2", "text3"]);
        assert_eq!(my_nested_list_row, my_nested_list);
    }
}

#[tokio::test]
#[cfg(all(feature = "e2e-tests"))]
async fn list_v4() {
    let cql = "CREATE TABLE IF NOT EXISTS cdrs_test.test_lists_v4 \
               (my_text_list frozen<list<text>> PRIMARY KEY, \
               my_nested_list list<frozen<list<smallint>>>)";
    let session = setup(cql).await.expect("setup");

    let my_text_list = vec![
        "text1".to_string(),
        "text2".to_string(),
        "text3".to_string(),
    ];
    let my_nested_list: Vec<Vec<i16>> =
        vec![vec![1, 2, 3], vec![999, 888, 777, 666, 555], vec![-1, -2]];
    let values = query_values!(my_text_list.clone(), my_nested_list.clone());

    let cql = "INSERT INTO cdrs_test.test_lists_v4 \
               (my_text_list, my_nested_list) VALUES (?, ?)";
    session
        .query_with_values(cql, values)
        .await
        .expect("insert");

    let cql = "SELECT * FROM cdrs_test.test_lists_v4";
    let rows = session
        .query(cql)
        .await
        .expect("query")
        .response_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    assert_eq!(rows.len(), 1);
    for row in rows {
        let my_text_list_row: Vec<String> = row
            .r_by_name::<List>("my_text_list")
            .expect("my_text_list")
            .as_r_rust()
            .expect("my_text_list as rust");
        let my_nested_list_outer_row: Vec<List> = row
            .r_by_name::<List>("my_nested_list")
            .expect("my_nested_list")
            .as_r_rust()
            .expect("my_nested_list (outer) as rust");
        let mut my_nested_list_row = Vec::with_capacity(my_nested_list_outer_row.len());
        for my_nested_list_inner_row in my_nested_list_outer_row {
            let my_nested_list_inner_row: Vec<i16> = my_nested_list_inner_row
                .as_r_rust()
                .expect("my_nested_list (inner) as rust");
            my_nested_list_row.push(my_nested_list_inner_row);
        }
        assert_eq!(my_text_list_row, my_text_list);
        assert_eq!(my_nested_list_row, my_nested_list);
    }
}

#[tokio::test]
#[cfg(feature = "e2e-tests")]
async fn set() {
    let cql = "CREATE TABLE IF NOT EXISTS cdrs_test.test_sets \
               (my_text_set frozen<set<text>> PRIMARY KEY, \
               my_nested_set set<frozen<set<int>>>)";
    let session = setup(cql).await.expect("setup");

    let my_text_set = vec![
        "text1".to_string(),
        "text2".to_string(),
        "text3".to_string(),
    ];
    let my_nested_set: Vec<Vec<i32>> =
        vec![vec![-2, -1], vec![1, 2, 3], vec![555, 666, 777, 888, 999]];
    let values = query_values!(my_text_set.clone(), my_nested_set.clone());

    let cql = "INSERT INTO cdrs_test.test_sets \
               (my_text_set, my_nested_set) VALUES (?, ?)";
    session
        .query_with_values(cql, values)
        .await
        .expect("insert");

    let cql = "SELECT * FROM cdrs_test.test_sets";
    let rows = session
        .query(cql)
        .await
        .expect("query")
        .response_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    assert_eq!(rows.len(), 1);
    for row in rows {
        let my_text_set_row: Vec<String> = row
            .r_by_name::<List>("my_text_set")
            .expect("my_text_set")
            .as_r_rust()
            .expect("my_text_set as rust");
        let my_nested_set_outer_row: Vec<List> = row
            .r_by_name::<List>("my_nested_set")
            .expect("my_nested_set")
            .as_r_rust()
            .expect("my_nested_set (outer) as rust");
        let mut my_nested_set_row = Vec::with_capacity(my_nested_set_outer_row.len());
        for my_nested_set_inner_row in my_nested_set_outer_row {
            let my_nested_set_inner_row: Vec<i32> = my_nested_set_inner_row
                .as_r_rust()
                .expect("my_nested_set (inner) as rust");
            my_nested_set_row.push(my_nested_set_inner_row);
        }
        assert_eq!(my_text_set_row, my_text_set);
        assert_eq!(my_nested_set_row, my_nested_set);
    }
}

#[tokio::test]
#[cfg(all(feature = "e2e-tests"))]
async fn set_v4() {
    let cql = "CREATE TABLE IF NOT EXISTS cdrs_test.test_sets_v4 \
               (my_text_set frozen<set<text>> PRIMARY KEY, \
               my_nested_set set<frozen<set<smallint>>>)";
    let session = setup(cql).await.expect("setup");

    let my_text_set = vec![
        "text1".to_string(),
        "text2".to_string(),
        "text3".to_string(),
    ];
    let my_nested_set: Vec<Vec<i16>> =
        vec![vec![-2, -1], vec![1, 2, 3], vec![555, 666, 777, 888, 999]];
    let values = query_values!(my_text_set.clone(), my_nested_set.clone());

    let cql = "INSERT INTO cdrs_test.test_sets_v4 \
               (my_text_set, my_nested_set) VALUES (?, ?)";
    session
        .query_with_values(cql, values)
        .await
        .expect("insert");

    let cql = "SELECT * FROM cdrs_test.test_sets_v4";
    let rows = session
        .query(cql)
        .await
        .expect("query")
        .response_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    assert_eq!(rows.len(), 1);
    for row in rows {
        let my_text_set_row: Vec<String> = row
            .r_by_name::<List>("my_text_set")
            .expect("my_text_set")
            .as_r_rust()
            .expect("my_text_set as rust");
        let my_nested_set_outer_row: Vec<List> = row
            .r_by_name::<List>("my_nested_set")
            .expect("my_nested_set")
            .as_r_rust()
            .expect("my_nested_set (outer) as rust");
        let mut my_nested_set_row = Vec::with_capacity(my_nested_set_outer_row.len());
        for my_nested_set_inner_row in my_nested_set_outer_row {
            let my_nested_set_inner_row: Vec<i16> = my_nested_set_inner_row
                .as_r_rust()
                .expect("my_nested_set (inner) as rust");
            my_nested_set_row.push(my_nested_set_inner_row);
        }
        assert_eq!(my_text_set_row, my_text_set);
        assert_eq!(my_nested_set_row, my_nested_set);
    }
}

#[tokio::test]
#[cfg(feature = "e2e-tests")]
async fn map_without_blob() {
    let cql = "CREATE TABLE IF NOT EXISTS cdrs_test.test_maps_without_blob \
               (my_key int PRIMARY KEY, \
               my_text_map map<text, text>, \
               my_nested_map map<uuid, frozen<map<bigint, int>>>)";
    let session = setup(cql).await.expect("setup");

    let my_text_map = hashmap! {
        "key1".to_string() => "value1".to_string(),
        "key2".to_string() => "value2".to_string(),
        "key3".to_string() => "value3".to_string(),
    };
    let my_nested_map: HashMap<Uuid, HashMap<i64, i32>> = hashmap! {
        Uuid::from_str("bb16106a-10bc-4a07-baa3-126ffe208c43").unwrap() => hashmap!{
            1 => 1,
            2 => 2,
        },
        Uuid::from_str("687d7677-dbf0-4d25-8cf3-e5d9185bba0b").unwrap() => hashmap!{
            1 => 1,
        },
        Uuid::from_str("c4dc6e8b-758a-4af4-ab00-ec356fb688d9").unwrap() => hashmap!{
            1 => 1,
            2 => 2,
            3 => 3,
        },
    };
    let values = query_values!(0i32, my_text_map.clone(), my_nested_map.clone());

    let cql = "INSERT INTO cdrs_test.test_maps_without_blob \
               (my_key, my_text_map, my_nested_map) VALUES (?, ?, ?)";
    session
        .query_with_values(cql, values)
        .await
        .expect("insert");

    let cql = "SELECT * FROM cdrs_test.test_maps_without_blob";
    let rows = session
        .query(cql)
        .await
        .expect("query")
        .response_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    assert_eq!(rows.len(), 1);
    for row in rows {
        let my_text_map_row: HashMap<String, String> = row
            .r_by_name::<Map>("my_text_map")
            .expect("my_text_map")
            .as_r_rust()
            .expect("my_text_map as rust");
        let my_nested_map_outer_row: HashMap<Uuid, Map> = row
            .r_by_name::<Map>("my_nested_map")
            .expect("my_nested_map")
            .as_r_rust()
            .expect("my_nested_map (outer) as rust");
        let mut my_nested_map_row = HashMap::with_capacity(my_nested_map_outer_row.len());
        for (index, my_nested_map_inner_row) in my_nested_map_outer_row {
            let my_nested_map_inner_row: HashMap<i64, i32> = my_nested_map_inner_row
                .as_r_rust()
                .expect("my_nested_map (inner) as rust");
            my_nested_map_row.insert(index, my_nested_map_inner_row);
        }
        assert_eq!(my_text_map_row, my_text_map);
        assert_eq!(my_nested_map_row, my_nested_map);
    }
}

#[tokio::test]
#[cfg(all(feature = "e2e-tests"))]
async fn map_without_blob_v4() {
    let cql = "CREATE TABLE IF NOT EXISTS cdrs_test.test_maps_without_blob_v4 \
               (my_text_map frozen<map<text, text>> PRIMARY KEY, \
               my_nested_map map<uuid, frozen<map<bigint, tinyint>>>)";
    let session = setup(cql).await.expect("setup");

    let my_text_map = hashmap! {
        "key1".to_string() => "value1".to_string(),
        "key2".to_string() => "value2".to_string(),
        "key3".to_string() => "value3".to_string(),
    };
    let my_nested_map: HashMap<Uuid, HashMap<i64, i8>> = hashmap! {
        Uuid::from_str("bb16106a-10bc-4a07-baa3-126ffe208c43").unwrap() => hashmap!{
            1 => 1,
            2 => 2,
        },
        Uuid::from_str("687d7677-dbf0-4d25-8cf3-e5d9185bba0b").unwrap() => hashmap!{
            1 => 1,
        },
        Uuid::from_str("c4dc6e8b-758a-4af4-ab00-ec356fb688d9").unwrap() => hashmap!{
            1 => 1,
            2 => 2,
            3 => 3,
        },
    };
    let values = query_values!(my_text_map.clone(), my_nested_map.clone());

    let cql = "INSERT INTO cdrs_test.test_maps_without_blob_v4 \
               (my_text_map, my_nested_map) VALUES (?, ?)";
    session
        .query_with_values(cql, values)
        .await
        .expect("insert");

    let cql = "SELECT * FROM cdrs_test.test_maps_without_blob_v4";
    let rows = session
        .query(cql)
        .await
        .expect("query")
        .response_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    assert_eq!(rows.len(), 1);
    for row in rows {
        let my_text_map_row: HashMap<String, String> = row
            .r_by_name::<Map>("my_text_map")
            .expect("my_text_map")
            .as_r_rust()
            .expect("my_text_map as rust");
        let my_nested_map_outer_row: HashMap<Uuid, Map> = row
            .r_by_name::<Map>("my_nested_map")
            .expect("my_nested_map")
            .as_r_rust()
            .expect("my_nested_map (outer) as rust");
        let mut my_nested_map_row = HashMap::with_capacity(my_nested_map_outer_row.len());
        for (index, my_nested_map_inner_row) in my_nested_map_outer_row {
            let my_nested_map_inner_row: HashMap<i64, i8> = my_nested_map_inner_row
                .as_r_rust()
                .expect("my_nested_map (inner) as rust");
            my_nested_map_row.insert(index, my_nested_map_inner_row);
        }
        assert_eq!(my_text_map_row, my_text_map);
        assert_eq!(my_nested_map_row, my_nested_map);
    }
}

#[tokio::test]
#[cfg(feature = "e2e-tests")]
async fn map() {
    let cql = "CREATE TABLE IF NOT EXISTS cdrs_test.test_maps \
               (my_text_map frozen<map<text, text>> PRIMARY KEY, \
               my_nested_map map<uuid, frozen<map<bigint, blob>>>)";
    let session = setup(cql).await.expect("setup");

    let my_text_map = hashmap! {
        "key1".to_string() => "value1".to_string(),
        "key2".to_string() => "value2".to_string(),
        "key3".to_string() => "value3".to_string(),
    };
    let my_nested_map: HashMap<Uuid, HashMap<i64, Blob>> = hashmap! {
        Uuid::from_str("bb16106a-10bc-4a07-baa3-126ffe208c43").unwrap() => hashmap!{
            1 => vec![52, 121, 209, 200, 81, 118, 181, 17].into(),
            2 => vec![226, 90, 51, 10, 26, 87, 141, 61].into(),
        },
        Uuid::from_str("687d7677-dbf0-4d25-8cf3-e5d9185bba0b").unwrap() => hashmap!{
            1 => vec![224, 155, 148, 6, 217, 96, 120, 38].into(),
        },
        Uuid::from_str("c4dc6e8b-758a-4af4-ab00-ec356fb688d9").unwrap() => hashmap!{
            1 => vec![164, 238, 196, 10, 149, 169, 145, 239].into(),
            2 => vec![250, 87, 119, 134, 105, 236, 240, 64].into(),
            3 => vec![72, 81, 26, 173, 107, 96, 38, 91].into(),
        },
    };
    let values = query_values!(my_text_map.clone(), my_nested_map.clone());

    let cql = "INSERT INTO cdrs_test.test_maps \
               (my_text_map, my_nested_map) VALUES (?, ?)";
    session
        .query_with_values(cql, values)
        .await
        .expect("insert");

    let cql = "SELECT * FROM cdrs_test.test_maps";
    let rows = session
        .query(cql)
        .await
        .expect("query")
        .response_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");

    assert_eq!(rows.len(), 1);
    for row in rows {
        let my_text_map_row: HashMap<String, String> = row
            .r_by_name::<Map>("my_text_map")
            .expect("my_text_map")
            .as_r_rust()
            .expect("my_text_map as rust");
        let my_nested_map_outer_row: HashMap<Uuid, Map> = row
            .r_by_name::<Map>("my_nested_map")
            .expect("my_nested_map")
            .as_r_rust()
            .expect("my_nested_map (outer) as rust");
        let mut my_nested_map_row = HashMap::with_capacity(my_nested_map_outer_row.len());
        for (index, my_nested_map_inner_row) in my_nested_map_outer_row {
            let my_nested_map_inner_row: HashMap<i64, Blob> = my_nested_map_inner_row
                .as_r_rust()
                .expect("my_nested_map (inner) as rust");
            my_nested_map_row.insert(index, my_nested_map_inner_row);
        }
        assert_eq!(my_text_map_row, my_text_map);
        assert_eq!(my_nested_map_row, my_nested_map);
    }
}

use crate::{Device, Home, Result, Room, SmartSocket, Thermometr};

async fn connect_to_smartsocket_mock() -> Device {
    let smartsocket_mock_addr = std::env::var_os("SMARTSOCKET_ADDR")
        .and_then(|os_string| os_string.into_string().ok())
        .and_then(|string| string.parse().ok())
        .unwrap_or_else(|| "127.0.0.1:55331".to_string());

    SmartSocket::connect(smartsocket_mock_addr)
        .await
        .expect("run smartsocket mock server from examples")
        .into()
}

async fn connect_to_thermometr_mock() -> Device {
    let thermometr_mock_addr = std::env::var_os("TERMOMETR_ADDR")
        .and_then(|os_string| os_string.into_string().ok())
        .and_then(|string| string.parse().ok())
        .unwrap_or_else(|| "127.0.0.1:55332".to_string());

    Thermometr::connect(thermometr_mock_addr)
        .await
        .expect("run thermometr mock server from examples")
        .into()
}

#[test]
// test for Дом имеет название и содержит несколько помещений
// here we test that pub field `name` is correctly set with `Home::new()`
fn home_has_name() {
    let home_hame = String::from("my home");
    let home = Home::new(home_hame.clone());
    assert_eq!(home_hame, home.name);
}

#[test]
// test for Библитотека позволяет запросить список помещений, добавлять и удалять помещения в доме.
// here we test that home has as many rooms as we expect after adding and removing it
fn home_has_rooms() -> Result<()> {
    let mut home = Home::new("my home".into());

    // test we can add rooms
    home.add_room("room1".into(), Room::default())?;
    home.add_room("room2".into(), Room::default())?;
    home.add_room("room3".into(), Room::default())?;

    // test that we have rooms we added
    let mut room_names: Vec<_> = home.room_names().collect();
    room_names.sort_unstable(); // we don't have any order requirements so here we sort before checking result
    match room_names[..] {
        ["room1", "room2", "room3"] => Ok(()),
        ref x => Err(format!("expected [room1, room2, room3], got {:?}", x)),
    }?;

    // test we can get rooms we added
    home.room("room1").ok_or("no room1")?;
    home.room("room2").ok_or("no room2")?;
    home.room("room3").ok_or("no room3")?;

    // test we can delete room
    home.delete_room("room2")?;

    // test that room was deleted
    let mut room_names: Vec<_> = home.room_names().collect();
    room_names.sort_unstable();
    match room_names[..] {
        ["room1", "room3"] => Ok(()),
        ref x => Err(format!("expected [room1, room3], got {:?}", x)),
    }?;

    // test we can get rooms we didn't delete
    home.room("room1").ok_or("no room1")?;
    home.room("room3").ok_or("no room3")?;

    // test we can't get room we deleted
    if home.room("room2").is_some() {
        return Err("Expected home.room(\"room2\") to return Err".into());
    };

    Ok(())
}

#[test]
// test for Помещение имеет уникальное название и содержит несколько устройств
// here we test that home does not allow to add room with not unique name
fn home_room_non_unique_name_isnt_allowed() {
    let mut home = Home::new("my home".into());

    home.add_room("room1".into(), Room::default()).ok(); // ok() to ignore result

    // next add_room call with the same name should return err
    assert!(home.add_room("room1".into(), Room::default()).is_err());
}

#[tokio::test]
// test for Устройство имеет уникальное в рамках помещения название, тип и описание.
// here we test that room does not allow non-unique device names
async fn room_device_non_unique_name_isnt_allowed() {
    let mut room = Room::default();

    room.add_device(
        "device1".into(),
        "".into(),
        connect_to_thermometr_mock().await,
    )
    .ok(); // ok() to ignore result

    // next add_device call with the same name should return err
    assert!(room
        .add_device(
            "device1".into(),
            "".into(),
            connect_to_smartsocket_mock().await
        )
        .is_err());
}

#[tokio::test]
// test for Устройство имеет уникальное в рамках помещения название, тип и описание.
// here we test that room does not allow non-unique device types
async fn room_device_non_unique_type_isnt_allowed() {
    let mut room = Room::default();

    room.add_device(
        "device1".into(),
        "".into(),
        connect_to_thermometr_mock().await,
    )
    .ok(); // ok() to ignore result

    // next add_device call with the same type should return err
    assert!(room
        .add_device(
            "device2".into(),
            "".into(),
            connect_to_thermometr_mock().await
        )
        .is_err());
}

#[tokio::test]
// test for Библтотека позволяет добавлять, получать и удалять любое устройство в доме. Получать список устройств в помещении.
// here we test we have as expected number of devices in a room
async fn room_has_devices() -> Result<()> {
    let mut room = Room::default();

    // test we can add devices
    room.add_device(
        "thermo".into(),
        "".into(),
        connect_to_thermometr_mock().await,
    )?;
    room.add_device(
        "socket".into(),
        "".into(),
        connect_to_smartsocket_mock().await,
    )?;

    // test that we have devices we added
    let mut device_names: Vec<_> = room.device_names().collect();
    device_names.sort_unstable(); // we don't have any order requirements so here we sort before checking result
    match device_names[..] {
        ["socket", "thermo"] => Ok(()),
        ref x => Err(format!("expected [socket, thermo], got {:?}", x)),
    }?;

    // test we can get devices we added
    room.device("thermo").ok_or("no thermo")?;
    room.device("socket").ok_or("no socket")?;

    // test we can delete device
    room.delete_device("thermo")?;

    // test that device was deleted
    let device_names: Vec<_> = room.device_names().collect();
    match device_names[..] {
        ["socket"] => Ok(()),
        ref x => Err(format!("expected [socket], got {:?}", x)),
    }?;

    // test we can get device we didn't delete
    room.device("socket").ok_or("no socket")?;

    // test we can't get device we deleted
    if room.device("thermo").is_some() {
        return Err("Expected room.device(\"thermo\") to return Err".into());
    };

    Ok(())
}

#[tokio::test]
//test for Типы устройств: термометр, умная розетка.
//here we test we can create device with type thermometr
async fn create_thermometr() -> Result<()> {
    match connect_to_thermometr_mock().await {
        Device::Thermometr(_) => Ok(()),
        device => Err(format!("Expected thermometr, got {:?}", device).into()),
    }
}

#[tokio::test]
//test for Типы устройств: термометр, умная розетка.
//here we test we can create device with type smartsocket
async fn create_smartsocket() -> Result<()> {
    match connect_to_smartsocket_mock().await {
        Device::SmartSocket(_) => Ok(()),
        device => Err(format!("Expected smartsocket, got {:?}", device).into()),
    }
}

#[tokio::test]
// test for Термометр позволяет узнать температуру
// here we just test that functions do not panic, cause we don't have requriements
async fn termometr_functions() {
    let d: Device = connect_to_thermometr_mock().await;

    assert!(d.state().await.is_ok(), "thermometr state is err");

    if let Device::Thermometr(t) = d {
        assert!(
            t.temperature().await.is_ok(),
            "run thermometr mock server from examples"
        );
    } else {
        unreachable!("d must be Device::Thermometr")
    };
}

#[tokio::test]
// test for Умная розетка позволяет включать и выключать себя. Предоставляет информацию о текущем состоянии и потребляемой мощности.
// here we just test that functions do not panic, cause we don't have requriements
async fn smartsocket_functions() {
    let d: Device = connect_to_smartsocket_mock().await;

    assert!(d.state().await.is_ok(), "smartsocket state is err");

    if let Device::SmartSocket(mut s) = d {
        s.switch(true).await.ok(); // shouldn't panic
        let (is_on, power_before) = s.state().await.unwrap();
        assert!(is_on, "is_on should be true after switch(true)");
        std::thread::sleep(std::time::Duration::from_secs(2));
        s.switch(false).await.ok(); // shouldn't panic
        let (is_on, power_after) = s.state().await.unwrap();
        assert!(!is_on, "is_on should be false after switch(false)");
        // smartsocket mock increases power by 1 every second, so
        assert!(
            power_after - power_before >= 1,
            "power usage should increase"
        )
    } else {
        unreachable!("d must be Device::SmartSocket")
    };
}

#[tokio::test]
// test for Библиотека позволяет строить отчёт о состоянии всех устройств в доме.
// here we just test that functions do not panic, cause we don't have requriements
async fn home_state_functions() {
    let mut home = Home::new("my home".into());
    let room_names = ["room1", "room2", "room3"];

    for rn in room_names {
        home.add_room(rn.into(), Room::default()).ok();

        home.room(rn)
            .unwrap()
            .add_device(
                "thermo".into(),
                "".into(),
                connect_to_thermometr_mock().await,
            )
            .ok();

        home.room(rn)
            .unwrap()
            .add_device(
                "socket".into(),
                "".into(),
                connect_to_smartsocket_mock().await,
            )
            .ok();
    }

    home.state().await; // shouldn't panic
}

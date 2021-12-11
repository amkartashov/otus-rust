use smarthome::{Device, Home, Room, SmartSocket, Thermometr};

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
fn home_has_rooms() -> Result<(), String> {
    let mut home = Home::new("my home".to_string());

    // test we can add rooms
    home.add_room("room1".to_string(), Room::default())?;
    home.add_room("room2".to_string(), Room::default())?;
    home.add_room("room3".to_string(), Room::default())?;

    // test that we have rooms we added
    let mut room_names = home.rooms();
    room_names.sort(); // we don't have any order requirements so here we sort before checking result
    match room_names[..] {
        ["room1", "room2", "room3"] => Ok(()),
        ref x => Err(format!("expected [room1, room2, room3], got {:?}", x)),
    }?;

    // test we can get rooms we added
    home.room("room1")?;
    home.room("room2")?;
    home.room("room3")?;

    // test we can delete room
    home.delete_room("room2")?;

    // test that room was deleted
    let mut room_names = home.rooms();
    room_names.sort();
    match room_names[..] {
        ["room1", "room3"] => Ok(()),
        ref x => Err(format!("expected [room1, room3], got {:?}", x)),
    }?;

    // test we can get rooms we didn't delete
    home.room("room1")?;
    home.room("room3")?;

    // test we can't get room we deleted
    if home.room("room2").is_ok() {
        return Err("Expected home.room(\"room2\") to return Err".to_string());
    };

    Ok(())
}

#[test]
// test for Помещение имеет уникальное название и содержит несколько устройств
// here we test that home does not allow to add room with not unique name
fn home_room_non_unique_name_isnt_allowed() {
    let mut home = Home::new("my home".to_string());

    home.add_room("room1".to_string(), Room::default()).ok(); // ok() to ignore result
                                                              // next add_room call with the same name should return err
    assert!(home.add_room("room1".to_string(), Room::default()).is_err());
}

#[test]
// test for Устройство имеет уникальное в рамках помещения название, тип и описание.
// here we test that room does not allow non-unique device names
fn room_device_non_unique_name_isnt_allowed() {
    let mut room = Room::default();

    room.add_device("device1".to_string(), "".to_string(), Thermometr::new())
        .ok(); // ok() to ignore result
               // next add_device call with the same name should return err
    assert!(room
        .add_device("device1".to_string(), "".to_string(), SmartSocket::new())
        .is_err());
}

#[test]
// test for Устройство имеет уникальное в рамках помещения название, тип и описание.
// here we test that room does not allow non-unique device types
fn room_device_non_unique_type_isnt_allowed() {
    let mut room = Room::default();

    room.add_device("device1".to_string(), "".to_string(), Thermometr::new())
        .ok(); // ok() to ignore result
               // next add_device call with the same type should return err
    assert!(room
        .add_device("device2".to_string(), "".to_string(), Thermometr::new())
        .is_err());
}

#[test]
// test for Библтотека позволяет добавлять, получать и удалять любое устройство в доме. Получать список устройств в помещении.
// here we test we have as expected number of devices in a room
fn room_has_devices() -> Result<(), String> {
    let mut room = Room::default();

    // test we can add devices
    room.add_device("thermo".to_string(), "".to_string(), Thermometr::new())?;
    room.add_device("socket".to_string(), "".to_string(), SmartSocket::new())?;

    // test that we have devices we added
    let mut device_names = room.devices();
    device_names.sort(); // we don't have any order requirements so here we sort before checking result
    match device_names[..] {
        ["thermo", "socket"] => Ok(()),
        ref x => Err(format!("expected [thermo, socket], got {:?}", x)),
    }?;

    // test we can get devices we added
    room.device("thermo")?;
    room.device("socket")?;

    // test we can delete device
    room.delete_device("thermo")?;

    // test that device was deleted
    let device_names = room.devices();
    match device_names[..] {
        ["socket"] => Ok(()),
        ref x => Err(format!("expected [socket], got {:?}", x)),
    }?;

    // test we can get device we didn't delete
    room.device("socket")?;

    // test we can't get device we deleted
    if room.device("thermo").is_ok() {
        return Err("Expected room.device(\"thermo\") to return Err".to_string());
    };

    Ok(())
}

#[test]
//test for Типы устройств: термометр, умная розетка.
//here we test we can create device with type thermometr
fn create_thermometr() -> Result<(), String> {
    match Thermometr::new() {
        Device::Thermometr(_) => Ok(()),
        device => Err(format!("Expected thermometr, got {:?}", device)),
    }
}

#[test]
//test for Типы устройств: термометр, умная розетка.
//here we test we can create device with type smartsocket
fn create_smartsocket() -> Result<(), String> {
    match SmartSocket::new() {
        Device::SmartSocket(_) => Ok(()),
        device => Err(format!("Expected smartsocket, got {:?}", device)),
    }
}

#[test]
// test for Термометр позволяет узнать температуру
// here we just test that functions do not panic, cause we don't have requriements
fn thermometr_functions() {
    let d = Thermometr::new();

    d.state(); // shouldn't panic

    if let Device::Thermometr(t) = d {
        t.temperature(); // shouldn't panic
    } else {
        unreachable!("d must be Device::Thermometr")
    };
}

#[test]
// test for Умная розетка позволяет включать и выключать себя. Предоставляет информацию о текущем состоянии и потребляемой мощности.
// here we just test that functions do not panic, cause we don't have requriements
fn smartsocket_functions() {
    let d = Thermometr::new();

    d.state(); // shouldn't panic

    if let Device::SmartSocket(mut s) = d {
        s.switch(true); // shouldn't panic
        s.switch(false); // shouldn't panic
        s.power(); // shouldn't panic
    } else {
        unreachable!("d must be Device::SmartSocket")
    };
}

#[test]
// test for Библиотека позволяет строить отчёт о состоянии всех устройств в доме.
// here we just test that functions do not panic, cause we don't have requriements
fn home_state_functions() {
    let mut home = Home::new("my home".to_string());
    let room_names = ["room1", "room2", "room3"];

    for rn in room_names {
        home.add_room(rn.to_string(), Room::default()).ok();

        home.room(rn)
            .unwrap()
            .add_device("thermo".to_string(), "".to_string(), Thermometr::new())
            .ok();

        home.room(rn)
            .unwrap()
            .add_device("socket".to_string(), "".to_string(), SmartSocket::new())
            .ok();
    }

    home.state(); // shouldn't panic
}

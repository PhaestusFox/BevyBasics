use bevy::{
    prelude::*,
    reflect::{GetPath, TypeRegistry},
};

pub struct ReflectExample;

impl Plugin for ReflectExample {
    fn build(&self, app: &mut App) {
        app.register_type::<ReflectStruct>();
        app.register_type::<ReflectEnum>();
        app.add_startup_system(your_system);
    }
}

#[derive(Debug, Clone, Reflect, serde::Serialize, serde::Deserialize)]
#[reflect(Serialize, Deserialize, DoThing)]
struct ReflectStruct {
    filed_0: usize,
    filed_1: usize,
    filed_2: ReflectEnum,
}

#[derive(Debug, Clone, Reflect, serde::Serialize, serde::Deserialize)]
#[reflect(Serialize, Deserialize)]
struct ReflectStructPath {
    filed_0: ReflectStruct,
    filed_1: ReflectStruct,
    filed_2: ReflectStruct,
}

#[derive(Debug, Clone, Reflect, serde::Serialize, serde::Deserialize)]
#[reflect(Serialize, Deserialize)]
enum ReflectEnum {
    Variant0,
    Variant1(String),
    Variant2(usize),
}

#[derive(Debug, Reflect, Clone, bevy::reflect::FromReflect)]
#[reflect_value()]
enum ReflectEnum2 {
    Value0,
    Value1,
    Value2,
}

fn get_field_by_name() {
    use bevy::reflect::ReflectRef;
    let hard_type = ReflectStruct {
        filed_0: 0,
        filed_1: 1,
        filed_2: ReflectEnum::Variant0,
    };
    let obj = hard_type.as_reflect();
    let name = "filed_0";
    let _filed = if let ReflectRef::Struct(data) = obj.reflect_ref() {
        //Some(&T) if it has field with this name
        //None if field is not of type T
        data.get_field::<usize>(name)
    } else {
        //do something if the field is not there
        todo!()
    };
    //or
    let name = "filed_1";
    let _filed = if let ReflectRef::Struct(data) = obj.reflect_ref() {
        //Some(&T) if it has field with this name
        //None if field is not of type T
        data.get_field::<usize>(name)
    } else {
        //do something if the field is not there
        todo!()
    };
}

fn patch_struct() {
    use bevy::reflect::DynamicStruct;
    let mut hard_type = ReflectStruct {
        filed_0: 0,
        filed_1: 1,
        filed_2: ReflectEnum::Variant0,
    };
    let mut dynamic_struct = DynamicStruct::default();
    dynamic_struct.insert::<usize>("filed_0", 1);
    hard_type.apply(dynamic_struct.as_reflect());
    assert_eq!(hard_type.filed_0, 1);
}

fn path_get() {
    let top = ReflectStructPath {
        filed_0: ReflectStruct {
            filed_0: 0,
            filed_1: 1,
            filed_2: ReflectEnum::Variant0,
        },
        filed_1: ReflectStruct {
            filed_0: 2,
            filed_1: 3,
            filed_2: ReflectEnum::Variant1("One".into()),
        },
        filed_2: ReflectStruct {
            filed_0: 4,
            filed_1: 5,
            filed_2: ReflectEnum::Variant2(2),
        },
    };
    if let Ok(val) = top.get_path::<usize>("filed_0.filed_1") {
        assert_eq!(*val, 1);
    };
    if let Ok(val) = top.get_path::<usize>("filed_2.filed_0") {
        assert_eq!(*val, 4);
    };
}

fn iterator() {
    use bevy::reflect::DynamicStruct;
    let mut zero = 0usize;
    let mut dynamic = DynamicStruct::default();
    dynamic.insert("num", 0usize);
    let mut hard_type = ReflectStruct {
        filed_0: 0,
        filed_1: 1,
        filed_2: ReflectEnum::Variant0,
    };
    fn add_one(to: &mut dyn Reflect) {
        use bevy::reflect::ReflectMut;
        match to.reflect_mut() {
            ReflectMut::Value(val) => {
                if val.is::<usize>() {
                    *val.downcast_mut::<usize>().unwrap() += 1;
                }
            }
            ReflectMut::Struct(struct_data) => {
                for field in 0..struct_data.field_len() {
                    let field = struct_data.field_at_mut(field).unwrap();
                    if field.is::<usize>() {
                        *field.downcast_mut::<usize>().unwrap() += 1;
                    }
                }
            }
            _ => todo!(),
        }
    }
    add_one(zero.as_reflect_mut());
    add_one(dynamic.as_reflect_mut());
    add_one(hard_type.as_reflect_mut());
    assert_eq!(zero, 1);
    assert_eq!(dynamic.get_field("num"), Some(&1usize));
    assert_eq!(hard_type.filed_0, 1);
    assert_eq!(hard_type.filed_1, 2);
}

fn reflect_trait() {
    let type_registure = TypeRegistry::default();
    let mut type_registure = type_registure.write();
    type_registure.register::<ReflectStruct>();
    let hard_type = ReflectStruct {
        filed_0: 0,
        filed_1: 0,
        filed_2: ReflectEnum::Variant0,
    };
    let obj = hard_type.as_reflect();
    let info = type_registure.get(obj.type_id()).unwrap();
    if let Some(do_thing) = info.data::<ReflectDoThing>() {
        let obj = do_thing.get(obj).unwrap();
        println!("{}", obj.do_thing())
    }
}

#[test]
fn serialize() {
    let mut world = World::default();
    world.spawn(Transform::default());
    let type_registry = AppTypeRegistry::default();
    {
        let mut type_registry = type_registry.write();
        type_registry.register::<Transform>();
        type_registry.register::<Vec3>();
        type_registry.register::<Quat>();
    }
    let scene = bevy::scene::DynamicScene::from_world(&world, &type_registry);
    let res = scene.serialize_ron(&type_registry);
    if let Ok(str) = res {
        println!("{}", str);
    }
}

fn scripting() {
    let mut data = ReflectStructPath {
        filed_0: ReflectStruct {
            filed_0: 0,
            filed_1: 1,
            filed_2: ReflectEnum::Variant0,
        },
        filed_1: ReflectStruct {
            filed_0: 2,
            filed_1: 3,
            filed_2: ReflectEnum::Variant1("One".into()),
        },
        filed_2: ReflectStruct {
            filed_0: 4,
            filed_1: 5,
            filed_2: ReflectEnum::Variant2(2),
        },
    };
    let user_script = "filed_0.filed_1 = 0";
    let mut word = user_script.split(' ');
    let filed = word.next();
    let op = word.next();
    let val = word.next();
    if let (Some(path), Some(op), Some(val)) = (filed, op, val) {
        let val: usize = val.parse().unwrap();
        if let Ok(old) = data.get_path_mut::<usize>(path) {
            match op {
                "=" => {
                    *old = val;
                }
                "+" => {
                    *old += val;
                }
                _ => todo!(),
            }
        }
    }
}

#[reflect_trait]
pub trait DoThing {
    fn do_thing(&self) -> String;
}
impl DoThing for ReflectStruct {
    fn do_thing(&self) -> String {
        "Hi I Reflect".to_string()
    }
}

// seems to be named AppTypeRegistry on the main branch
fn your_system(registry: Res<AppTypeRegistry>) {
    use std::any::TypeId;
    let registry = registry.read();
    if let Some(info) = registry.get(TypeId::of::<ReflectStruct>()) {
        println!("name: {}", info.short_name());
        println!("type info: {:?}", info.type_info());
        println!("type data: {:?}", info.data::<ReflectSerialize>().is_some());
        let array = [
            ReflectEnum2::Value0,
            ReflectEnum2::Value1,
            ReflectEnum2::Value2,
        ];
        let _array = array.as_reflect();
        let list = vec![
            ReflectEnum2::Value0,
            ReflectEnum2::Value1,
            ReflectEnum2::Value2,
        ];
        let _list = list.as_reflect();
        let _d_struct = bevy::reflect::DynamicStruct::default();
        let data = info.data::<ReflectSerialize>();
        let d = if let Some(data) = data {
            let reflect: Box<dyn Reflect> = Box::new(ReflectStruct {
                filed_0: 10,
                filed_1: 0,
                filed_2: ReflectEnum::Variant1("Test".to_string()),
            });
            let s = data.get_serializable(&*reflect);
            if let Ok(val) =
                ron::ser::to_string_pretty(s.borrow(), ron::ser::PrettyConfig::default())
            {
                println!("serialzie: {}", val);
                val
            } else {
                return;
            }
        } else {
            return;
        };
        let de = if let Some(de) = info.data::<ReflectDeserialize>() {
            de
        } else {
            println!("no deserialize");
            return;
        };
        let mut d = if let Ok(d) = ron::de::Deserializer::from_str(&d) {
            d
        } else {
            println!("ron deserializer failed");
            return;
        };
        let val = if let Ok(val) = de.deserialize(&mut d) {
            val
        } else {
            println!("deserialize failed");
            return;
        };
        match val.reflect_ref() {
            bevy::reflect::ReflectRef::Struct(info) => {
                for (i, v) in info.iter_fields().enumerate() {
                    println!("{:?}:{:?}", info.name_at(i), v);
                }
            }
            bevy::reflect::ReflectRef::TupleStruct(_) => todo!(),
            bevy::reflect::ReflectRef::Tuple(_) => todo!(),
            bevy::reflect::ReflectRef::List(_) => todo!(),
            bevy::reflect::ReflectRef::Array(_) => todo!(),
            bevy::reflect::ReflectRef::Map(_) => todo!(),
            bevy::reflect::ReflectRef::Value(_) => todo!(),
            bevy::reflect::ReflectRef::Enum(_) => todo!(),
        }
        let name = val.get_path::<ReflectEnum>("filed_2");
        println!("{:?}", name);
        let val: ReflectStruct = if let Ok(val) = val.take() {
            val
        } else {
            println!("downcast failed");
            return;
        };
        println!("{:?}", val)
    }
}

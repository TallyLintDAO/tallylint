#[macro_export]
macro_rules! generate_query_call {
  ($method_name:ident) => {
    #[allow(dead_code)]
    pub fn $method_name(
      env: &pocket_ic::PocketIc,
      sender: candid::Principal,
      canister_id: candid::Principal,
      args: &$method_name::Args,
    ) -> $method_name::Response {
      let method_name = stringify!($method_name);

      $crate::client::execute_query(env, sender, canister_id, method_name, args)
    }
  };
}

/*
what is MACRO : **is a rust code that generate rust code **

This code is generate a function of **different function args** of update call.
such as :
dfx canister call backend  query_a_wallet "10002"
dfx canister call backend  update_wallet '(record { id = 10002; name = "cczz";})'
its func-arg is diff

! need to IMPL xx_method::Args field
*/
#[macro_export] // attr to make this macro usable for other module
macro_rules! generate_update_call {
  //declares a new macro
  ($method_name:ident) => {
    //is the input to the macro.and binds it to the variable $method_name
    // (AKA:function name ) => : the code that will be generated.
    #[allow(dead_code)]
    pub fn $method_name(
      env: &mut pocket_ic::PocketIc,
      sender: candid::Principal,
      canister_id: candid::Principal,
      args: &$method_name::Args,
    ) -> $method_name::Response {
      let method_name = stringify!($method_name);

      // $crate: is a special variable that refers to the current crate
      $crate::client::execute_update(
        env,
        sender,
        canister_id,
        method_name,
        args,
      )
    }
  };
}

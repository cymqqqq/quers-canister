# ic_portal_canisters

#  current interface (process)
### Struct Profile

```rust
pub struct Profile {
    pub url: Url,
    pub pid: PrincipalId,
    pub aid: AccountId,
    pub nickname: Nickname,
    pub desc: Description,
    pub level: Level,
    pub state: State,
}
```
# implementations
## Profile

### fn register(&mut self, nickname: String) -> Result<Profile,InsertError> 
update method in candid type, just input your nickname(string) then you can see it in main page.
### example
```
dfx canister call ic_portal_canisters register '("Jack")'
``` 
### fn set_url(&mut self, url: String) -> Result<(), SetUrlError>
update method in candid type, the user need to input image url.
### example
```
dfx canister call ic_portal_canisters set_url '("www.ipfs.com")'
```
### fn update(&mut self, nickname: String, desc: String)
update method in candid type, the user need to input nickname(string) and description(string) to update the old one.
### example
```
dfx canister call ic_portal_canisters update '("Bod", "Bod is marching on the road")'
```
### fn set_description(&mut self, desc: String) -> Result<(), SetDescriptionError>
update method in candid type, to set you profile description
### example
```
dfx canister call ic_portal_canisters set_description '("Jack is coming")'
```
### fn get_url(&self) -> Result<Url,GetUrlError>
query method in candid type, to obtain the url
### example
```
dfx canister call ic_portal_canisters get_url
```
### fn get_description(&self) -> Result<Description,GetDescriptionError>
query method in candid type, to obtain your profile description
### example
```
dfx canister call ic_portal_canisters get_description
```
### fn get_one(&self) -> Result<Profile,GetError>
query method in candid type, to obtain your profile info about url, pid, aid, nickname, description, level, and state.
### example
```
dfx canister call ic_portal_canisters get_profile
```
### fn get_nickname(&self) -> Result<Nickname,GetNickNameError>
query method in candid type, to obtain your nickname when register.
### example
```
dfx canister call ic_portal_canisters get_nickname
```
### fn get_level(&self) -> Result<Level,GetLevelError>
query method in candid type, to obtain your nickname, it will be initialized to null string.
### example
```
dfx canister call ic_portal_canisters get_level
```
### fn get_state(&self) -> Result<State,GetStateError>
query method in candid type, to obtain your current state, it will be initialized to
"online"
### example
```
dfx canister call ic_portal_canisters get_state
```

## Friend
### fn add(&mut self, fid: String) -> (String, FriendList)
update method in candid type, to add a friend information with principal id in profile contract
### example
```
dfx canister call ic_portal_canisters add '("principal of friend")'
```
### fn delete(&mut self, fid: String)
update method in candid type, to delete a friend information with principal of friend
### example
```
dfx canister call ic_portal_canisters delete '("principal of friend")'
```
### fn display(&self) -> (String, FriendList)
query method in candid type, to display all friend list of current user
### example
```
dfx canister call ic_portal_canisters display
```
### fn clear(&mut self)
update method in candid type, to clear friend list of current user
### example
```
dfx canister call ic_portal_canisters clear
```
If you want to start working on your project right away, you might want to try the following commands:

```bash
cd ic_portal_canisters/
dfx help
dfx config --help
```
## Upgrading the Canister

If you want test your project upgradable, please ensure that wasam has been change

```bash
dfx build ic_portal_canisters 
dfx canister install ic_portal_canisters --mode=upgrade

```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

# canister id
dev asjdw-4qaaa-aaaah-abboq-cai
test ygrqs-jyaaa-aaaah-abihq-cai
product zdldy-bqaaa-aaaah-abdqq-cai
# notes
when you have installed rust, if you test in local environment, first run 'dfx start --background', before run 'dfx deploy',
remember to run 'rustup target add wasm32-unknown-unknown'
when you done this, everythig is going to be ok.

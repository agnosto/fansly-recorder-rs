# Fansly Recorder RS

A rewrite of someone's (mine) dogshit [python app](https://github.com/agnosto/fansly-recorder) in rust.
Was there a reason or need to have this written in rust? Not really but why not

## Requirements

- [ffmpeg](https://ffmpeg.org/) - [adding to path](https://phoenixnap.com/kb/ffmpeg-windows) on windows*.
- [cargo/rust](https://rustup.rs/)

## Build/Install 

Prebuilt binaries should be available [here](https://github.com/agnosto/fansly-recorder-rs/releases), should be able to simply unzip the file to get the executable.

### Build:

1. Clone the repo:

```bash
git clone https://github.com/agnosto/fansly-recorder-rs && cd fansly-recorder-rs
```

2. Build: 

```bash
cargo build --release
```

The executable should be available in `./target/release/fansly-recorder`


## Usage

On first run, the program will create a default config file for you to edit and add your fansly auth token to in order to handle all the request needed.


### Get fansly account token

#### Method 1 (Recommended) special thanks to [prof79](https://github.com/prof79/)'s wiki for this:
1. Go to [fansly](https://fansly.com) and login and open devtools (ctrl+shift+i / F12)
2. In devtools, go to the Console Tab and Paste the following: 
```javascript
console.clear(); // cleanup console
const activeSession = localStorage.getItem("session_active_session"); // get required key
const { token } = JSON.parse(activeSession); // parse the json data
console.log('%c➡️ Authorization_Token =', 'font-size: 12px; color: limegreen; font-weight: bold;', token); // show token
console.log('%c➡️ User_Agent =', 'font-size: 12px; color: yellow; font-weight: bold;', navigator.userAgent); // show user-agent
```

#### Method 2:
1. Go to [fansly](https://fansly.com) and login and open devtools (ctrl+shift+i / F12)
2. In network request, type `method:GET api` and click one of the requests
3. Look under `Request Headers` and look for `Authorization` and copy the value

#### Method 3:
1. Go to [fansly](https://fansly.com) and login and open devtools (ctrl+shift+i / F12)
2. Click on `Storage` and then `Local Storage`
3. Look for `session_active_session` and copy the `token` value


### Running 

```bash
fansly-recorder {username}
```

## TODO:

- [ ] Add check for ffmpeg before attempting to run 
- [ ] Add webhook sending for notifications of lives starting
- [ ] Add config option for recorded file ext to allow direct recording to mp4
- [ ] Maybe readd uploading to a remote host


# Super Serious And Needed Disclaimer

> "Fansly" is operated by Select Media LLC 👺.
>
> This repository and the provided content in it isn't in any way affiliated with, sponsored by, or endorsed by Select Media LLC or "Fansly" 👺.
>
> The developer of this script is not responsible for the end users' actions 👺.

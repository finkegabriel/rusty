RUSTY 

A rust robotics "library/framework" in it's infancy

- tested on raspberry pi zero

referenced this link for cross compiling to the pi zero

https://rust.azdevs.org/2019-07-24/

commands to build && transfer binary

cargo build --target=arm-unknown-linux-gnueabihf

scp target/arm-unknown-linux-gnueabihf/debug/rusty pi@raspberrypi.local:~/ 

created alias for linux 

`alias rusty='cd ~/rusty && cargo build --target=arm-unknown-linux-gnueabihf && scp target/arm-unknown-linux-gnueabihf/debug/rusty pi@raspberrypi.local:~/`

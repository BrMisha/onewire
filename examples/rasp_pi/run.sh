DESTINATION=pi@192.168.1.182

cross build --release --target armv7-unknown-linux-gnueabihf
rsync target/armv7-unknown-linux-gnueabihf/release/rasp_pi ${DESTINATION}:/home/pi/
ssh ${DESTINATION} /home/pi/rasp_pi
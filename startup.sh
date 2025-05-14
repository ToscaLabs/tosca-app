# The main router is the router connected to an Internet Service
# Provider in order to access to the Internet.
# It is the router **already** present in an environment prior to our arrival.
#
# The controller is the router where our web controller app is running on. It
# is installed in the environment by us and it serves both as a main manager 
# for our services, but also as a defensive layer towards other services which
# might be present in the environment.

# 1. Connect the LAN port of the main router with the WAN port of
# the controller router.
# 2. The IPV4 address of a controller router is dynamic and assigned through
# DHCP by the main router.
#
# The controller router defines a new network where each device must be
# connected.
# The controller router exposes this network so each device can connect to it.
# Each device MUST connect to the controller network because 

# Port 8123 cannot be busy because I'm the one who had configured the device.
#
# Run web controller app on the right IP address and with the right port.
./web-controller --ip 127.0.0.1 --port 8123

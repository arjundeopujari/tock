import serial
import time
import io

# Open the actual serial port
sp = serial.Serial()

sp.port = '/dev/ttyACM0'
sp.baudrate = 115200
sp.parity=serial.PARITY_NONE
sp.stopbits=1
sp.xonxoff=0
sp.rtscts=0
sp.timeout=1
# Try to set initial conditions, but not all platforms support them.
# https://github.com/pyserial/pyserial/issues/124#issuecomment-227235402
sp.dtr = 0
sp.rts = 0

sp.open()

time.sleep(0.1)

while True:
	sp.write("status".encode())
	print(sp.read(10))

	





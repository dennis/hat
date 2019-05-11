all:
	gcc -Wall -Wpedantic -c btlepasvscan.c
	gcc -Wall -Wpedantic -c blescan.c
	gcc btlepasvscan.o blescan.o -o blescan -lbluetooth

clean:
	-rm *.o

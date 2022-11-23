PHONY += comm
comm:
	make -C /lib/modules/$(shell uname -r)/build/ M=$(PWD)  modules
	$(CC) main.c -o main
clean:
	make -C /lib/modules/$(shell uname -r)/build/ M=$(PWD) clean
	rm main

.PHONY: $(PHONY)

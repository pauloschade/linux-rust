PHONY += comm
comm:
	make -C /lib/modules/$(shell uname -r)/build/ M=$(PWD)  modules
	$(CC) simple.c -o simple
clean:
	make -C /lib/modules/$(shell uname -r)/build/ M=$(PWD) clean
	rm simple

.PHONY: $(PHONY)

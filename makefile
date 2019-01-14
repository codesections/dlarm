.PHONY : install
install : 
	install -m 0755 -v ./target/release/dlarm /usr/local/bin/dlarm
	dlarm generate man > /usr/local/share/man/man1/dlarm.1
	install -d /usr/share/zsh/site-functions
	dlarm generate zsh > _dlarm
	mv _dlarm /usr/share/zsh/site-functions/_dlarm


.PHONY : uninstall
uninstall :
	rm -f /usr/local/bin/dlarm
	rm -f /usr/local/share/man/man1/dlarm.1
	rm -f /usr/share/zsh/site-functions/_dlarm


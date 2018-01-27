AUTOMAKE_OPTIONS = cygnus
INCLUDES =  -I$(srcdir)/include $(NEWLIB_CFLAGS) $(CROSS_CFLAGS) $(TARGET_CFLAGS)
AM_CCASFLAGS = $(INCLUDES)

noinst_LIBRARIES = lib.a

lib_a_LIBADD =
lib_a_SOURCES = \
    crt0.c
lib_a_DEPENDENCIES =
lib_a_CCASFLAGS = $(AM_CCASFLAGS)
lib_a_CFLAGS = $(AM_CFLAGS)

rust_sources = \
    src/lib.rs \
    src/macros.rs \
    src/event.rs \
    src/process.rs \
    src/file.rs \
    src/time.rs \
    src/folder.rs \
    src/unimpl.rs \
    src/user.rs \
    src/socket.rs \
    src/types.rs \
    src/hostname.rs \
    src/mallocnull.rs \
    src/rawfile.rs \
    src/netdb.rs \
    src/dns/answer.rs \
    src/dns/mod.rs \
    src/dns/query.rs

$(srcdir)/target/x86_64-unknown-redox/release/libnewlib_redox.a: Xargo.toml Cargo.toml $(rust_sources)
	cd $(srcdir); cargo update
	cd $(srcdir); CC=x86_64-unknown-redox-gcc CFLAGS="$(CFLAGS) -I $(srcdir)/../../include" xargo build --target x86_64-unknown-redox --release

lib.a: $(srcdir)/target/x86_64-unknown-redox/release/libnewlib_redox.a
	cp $(srcdir)/target/x86_64-unknown-redox/release/libnewlib_redox.a lib.a

if MAY_SUPPLY_SYSCALLS
all: crt0.o
endif

ACLOCAL_AMFLAGS = -I ../../..
CONFIG_STATUS_DEPENDENCIES = $(newlib_basedir)/configure.host

include $(srcdir)/../../../Makefile.shared

install-data-local:
	$(mkinstalldirs) $(DESTDIR)$(tooldir)/include/arpa;
	$(mkinstalldirs) $(DESTDIR)$(tooldir)/include/netinet;

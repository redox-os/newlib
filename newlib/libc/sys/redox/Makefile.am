AUTOMAKE_OPTIONS = cygnus
INCLUDES =  -I$(srcdir)/include -I$(srcdir)/pthreads-emb $(NEWLIB_CFLAGS) $(CROSS_CFLAGS) $(TARGET_CFLAGS)
AM_CCASFLAGS = $(INCLUDES)

noinst_LIBRARIES = lib.a

extra_objs = \
    pthreads-emb/pthread_mutex_unlock.o \
    pthreads-emb/pthread_mutex_init.o \
    pthreads-emb/pthread_mutex_destroy.o \
    pthreads-emb/pthread_mutex_lock.o \
    pthreads-emb/pthread_mutex_timedlock.o \
    pthreads-emb/pthread_mutex_trylock.o \
    pthreads-emb/pthread_mutexattr_destroy.o \
    pthreads-emb/pthread_mutexattr_getkind_np.o \
    pthreads-emb/pthread_mutexattr_getpshared.o \
    pthreads-emb/pthread_mutexattr_gettype.o \
    pthreads-emb/pthread_mutexattr_init.o \
    pthreads-emb/pthread_mutexattr_setkind_np.o \
    pthreads-emb/pthread_mutexattr_setpshared.o \
    pthreads-emb/pthread_mutexattr_settype.o \
    pthreads-emb/pte_relmillisecs.o \
    pthreads-emb/pte_mutex_check_need_init.o \
    pthreads-emb/pte_threadDestroy.o \
    pthreads-emb/pte_new.o \
    pthreads-emb/pte_threadStart.o \
    pthreads-emb/global.o \
    pthreads-emb/pte_reuse.o \
    pthreads-emb/pthread_init.o \
    pthreads-emb/pthread_terminate.o \
    pthreads-emb/create.o \
    pthreads-emb/pthread_self.o \
    pthreads-emb/pthread_equal.o \
    pthreads-emb/pthread_join.o \
    pthreads-emb/pthread_detach.o \
    pthreads-emb/pte_detach.o \
    pthreads-emb/pte_callUserDestroyRoutines.o \
    pthreads-emb/pte_tkAssocDestroy.o \
    pthreads-emb/pthread_kill.o \
    pthreads-emb/pthread_attr_destroy.o \
    pthreads-emb/pthread_attr_getdetachstate.o \
    pthreads-emb/pthread_attr_getinheritsched.o \
    pthreads-emb/pthread_attr_getschedparam.o \
    pthreads-emb/pthread_attr_getschedpolicy.o \
    pthreads-emb/pthread_attr_getscope.o \
    pthreads-emb/pthread_attr_getstackaddr.o \
    pthreads-emb/pthread_attr_getstacksize.o \
    pthreads-emb/pthread_attr_init.o \
    pthreads-emb/pthread_attr_setdetachstate.o \
    pthreads-emb/pthread_attr_setinheritsched.o \
    pthreads-emb/pthread_attr_setschedparam.o \
    pthreads-emb/pthread_attr_setschedpolicy.o \
    pthreads-emb/pthread_attr_setscope.o \
    pthreads-emb/pthread_attr_setstackaddr.o \
    pthreads-emb/pthread_attr_setstacksize.o \
    pthreads-emb/pte_is_attr.o \
    pthreads-emb/pthread_exit.o \
    pthreads-emb/pthread_getschedparam.o \
    pthreads-emb/pthread_setschedparam.o \
    pthreads-emb/sched_get_priority_max.o \
    pthreads-emb/sched_get_priority_min.o \
    pthreads-emb/pthread_key_create.o \
    pthreads-emb/pthread_key_delete.o \
    pthreads-emb/pthread_getspecific.o \
    pthreads-emb/pthread_setspecific.o \
    pthreads-emb/pte_tkAssocCreate.o \
    pthreads-emb/sched_yield.o \
    pthreads-emb/pthread_delay_np.o \
    pthreads-emb/pthread_testcancel.o \
    pthreads-emb/pte_throw.o \
    pthreads-emb/cleanup.o \
    pthreads-emb/pthread_once.o \
    pthreads-emb/pthread_num_processors_np.o \
    pthreads-emb/pte_getprocessors.o \
    pthreads-emb/pte_spinlock_check_need_init.o \
    pthreads-emb/global.o \
    pthreads-emb/pthread_timechange_handler_np.o \
    pthreads-emb/pte_cond_check_need_init.o \
    pthreads-emb/pthread_getconcurrency.o \
    pthreads-emb/pthread_setconcurrency.o \
    pthreads-emb/pte_cancellable_wait.o \
    pthreads-emb/sem_close.o \
    pthreads-emb/sem_destroy.o \
    pthreads-emb/sem_getvalue.o \
    pthreads-emb/sem_init.o \
    pthreads-emb/sem_open.o  \
    pthreads-emb/sem_post.o \
    pthreads-emb/sem_post_multiple.o \
    pthreads-emb/sem_timedwait.o \
    pthreads-emb/sem_trywait.o \
    pthreads-emb/sem_unlink.o \
    pthreads-emb/sem_wait.o \
    pthreads-emb/pthread_barrier_init.o \
    pthreads-emb/pthread_barrier_destroy.o \
    pthreads-emb/pthread_barrier_wait.o \
    pthreads-emb/pthread_barrierattr_init.o \
    pthreads-emb/pthread_barrierattr_destroy.o \
    pthreads-emb/pthread_barrierattr_getpshared.o \
    pthreads-emb/pthread_barrierattr_setpshared.o \
    pthreads-emb/pthread_spin_destroy.o \
    pthreads-emb/pthread_spin_init.o \
    pthreads-emb/pthread_spin_lock.o \
    pthreads-emb/pthread_spin_trylock.o \
    pthreads-emb/pthread_spin_unlock.o \
    pthreads-emb/pthread_cond_destroy.o \
    pthreads-emb/pthread_cond_init.o \
    pthreads-emb/pthread_cond_signal.o \
    pthreads-emb/pthread_cond_wait.o \
    pthreads-emb/pthread_condattr_destroy.o \
    pthreads-emb/pthread_condattr_getpshared.o \
    pthreads-emb/pthread_condattr_init.o \
    pthreads-emb/pthread_condattr_setpshared.o \
    pthreads-emb/pthread_rwlock_init.o \
    pthreads-emb/pthread_rwlock_destroy.o \
    pthreads-emb/pthread_rwlock_rdlock.o \
    pthreads-emb/pthread_rwlock_timedrdlock.o \
    pthreads-emb/pthread_rwlock_timedwrlock.o \
    pthreads-emb/pthread_rwlock_tryrdlock.o \
    pthreads-emb/pthread_rwlock_trywrlock.o \
    pthreads-emb/pthread_rwlock_unlock.o \
    pthreads-emb/pthread_rwlock_wrlock.o \
    pthreads-emb/pthread_rwlockattr_init.o \
    pthreads-emb/pthread_rwlockattr_destroy.o \
    pthreads-emb/pthread_rwlockattr_getpshared.o \
    pthreads-emb/pthread_rwlockattr_setpshared.o \
    pthreads-emb/pte_rwlock_check_need_init.o \
    pthreads-emb/pte_rwlock_cancelwrwait.o \
    pthreads-emb/pthread_cancel.o \
    pthreads-emb/pthread_setcanceltype.o \
    pthreads-emb/pthread_setcancelstate.o

lib_a_LIBADD = $(extra_objs)
lib_a_SOURCES = \
    crt0.c
lib_a_DEPENDENCIES = $(extra_objs)
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
    src/threads.rs \
    src/dns/answer.rs \
    src/dns/mod.rs \
    src/dns/query.rs

$(srcdir)/target/x86_64-unknown-redox/release/libnewlib_redox.a: Xargo.toml Cargo.toml $(rust_sources)
	cd $(srcdir); cargo update
	cd $(srcdir); CC=x86_64-unknown-redox-gcc CFLAGS="$(CFLAGS) -I $(srcdir)/../../include" xargo build --target x86_64-unknown-redox --release


lib.a: $(srcdir)/target/x86_64-unknown-redox/release/libnewlib_redox.a $(extra_objs)
	cp $< $@
	ar rs $< $(notdir $(extra_objs))

if MAY_SUPPLY_SYSCALLS
all: crt0.o
endif

ACLOCAL_AMFLAGS = -I ../../..
CONFIG_STATUS_DEPENDENCIES = $(newlib_basedir)/configure.host

include $(srcdir)/../../../Makefile.shared

install-data-local:
	$(mkinstalldirs) $(DESTDIR)$(tooldir)/include/arpa;
	$(mkinstalldirs) $(DESTDIR)$(tooldir)/include/netinet;

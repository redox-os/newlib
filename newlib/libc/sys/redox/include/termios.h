/*
 * termios.h
 *
 * Contains the definitions used by the terminal I/O interfaces.
 */

#ifndef _TERMIOS_H
#define _TERMIOS_H

#include <sys/types.h>

typedef unsigned char  cc_t;
typedef unsigned int   speed_t;
typedef unsigned int   tcflag_t;

#define VEOF       0
#define VEOL       1
#define VEOL2      2
#define VERASE     3
#define VWERASE    4
#define VKILL      5
#define VREPRINT   6
#define VSWTC      7
#define VINTR      8
#define VQUIT      9
#define VSUSP     10
#define VSTART    12
#define VSTOP     13
#define VLNEXT    14
#define VDISCARD  15
#define VMIN      16
#define VTIME     17

// flags for input modes
#define IGNBRK 000001
#define BRKINT 000002
#define IGNPAR 000004
#define PARMRK 000010
#define INPCK  000020
#define ISTRIP 000040
#define INLCR  000100
#define IGNCR  000200
#define ICRNL  000400
#define IXON   001000
#define IXOFF  002000
#define IXANY  004000

// flags for output modes
#define OPOST  000001
#define ONLCR  000002
#define OLCUC  000004
#define OCRNL  000010
#define ONOCR  000020
#define ONLRET 000040
#define OFILL  0000100
#define OFDEL  0000200

// baud rates
#define B0       000000
#define B50      000001
#define B75      000002
#define B110     000003
#define B134     000004
#define B150     000005
#define B200     000006
#define B300     000007
#define B600     000010
#define B1200    000011
#define B1800    000012
#define B2400    000013
#define B4800    000014
#define B9600    000015
#define B19200   000016
#define B38400   000017
#define B57600   000020
#define B115200  000021
#define B230400  000022
#define B460800  000023
#define B500000  000024
#define B576000  000025
#define B921600  000026
#define B1000000 000027
#define B1152000 000030
#define B1500000 000031
#define B2000000 000032
#define B2500000 000033
#define B3000000 000034
#define B3500000 000035
#define B4000000 000036

// control modes
#define CSIZE   0001400
#define   CS5   0000000
#define   CS6   0000400
#define   CS7   0001000
#define   CS8   0001400
#define CSTOPB  0002000
#define CREAD   0004000
#define PARENB  0010000
#define PARODD  0020000
#define HUPCL   0040000
#define CLOCAL  0100000

// local modes
#define ISIG    0x00000080
#define ICANON  0x00000100
#define ECHO    0x00000008
#define ECHOE   0x00000002
#define ECHOK   0x00000004
#define ECHONL  0x00000010
#define NOFLSH  0x80000000
#define TOSTOP  0x00400000
#define IEXTEN  0x00000400

// tcsetattr() args
#define TCSANOW   0x0001
#define TCSADRAIN 0x0002
#define TCSAFLUSH 0x0004

// tcflush() args
#define TCIFLUSH  0x0001
#define TCIOFLUSH 0x0003
#define TCOFLUSH  0x0002

// tcflow() args
#define TCIOFF    0x0001
#define TCION     0x0002
#define TCOOFF    0x0004
#define TCOON     0x0008

#define NCCS 32

struct termios {
	tcflag_t c_iflag;
	tcflag_t c_oflag;
	tcflag_t c_cflag;
	tcflag_t c_lflag;
	cc_t     c_cc[NCCS];
};

speed_t cfgetispeed(const struct termios *);
speed_t cfgetospeed(const struct termios *);
int     cfsetispeed(struct termios *, speed_t);
int     cfsetospeed(struct termios *, speed_t);
int     tcdrain(int);
int     tcflow(int, int);
int     tcflush(int, int);
int     tcgetattr(int, struct termios *);
pid_t   tcgetsid(int);
int     tcsendbreak(int, int);
int     tcsetattr(int, int, struct termios *);

#endif

#ifndef __OS_H__
#define __OS_H__


#include "../02-memanagement/types.h"
#include "../02-memanagement/platform.h"

#include <stddef.h>
#include <stdarg.h>

/* uart */
extern int uart_putc(char ch);
extern void uart_puts(char *s);

/* printf */
extern int printf(const char* s, ...);
extern void painc(char *s);

/*memory management */
extern int *page_alloc(int npages);
extern void page_free(void *p);

#endif /* __OS_H__ */

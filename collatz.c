#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <limits.h>

#define MAX ULLONG_MAX
#define MAX_DIV_2 (MAX/2ULL)
#define MAX_DIV_2_PLUS_1 (MAX_DIV_2+1ULL)

typedef struct {
  u_quad_t hi;
  u_quad_t lo;
} u128_t;

static u_quad_t mi = 0ULL;
static u128_t mj = {0ULL,0ULL};
static unsigned int mct = 0;

void div2(u128_t *i) {
  if (i->hi & 1ULL) { // i.hi is odd
    i->hi >>= 1;
    i->lo >>= 1;
    i->lo += MAX_DIV_2_PLUS_1;
  } else {
    i->hi >>= 1;
    i->lo >>= 1;
  }
}

void add(u128_t *i, u_quad_t hi, u_quad_t lo) {
  if (MAX - i->lo >= lo) {
    i->hi += hi;
    i->lo += lo;
  } else {
    i->hi += hi + 1;
    i->lo += lo - (MAX - i->lo) - 1ULL;
  }
}

void mul3plus1(u128_t *i) {
  u_quad_t hi = i->hi;
  u_quad_t lo = i->lo;
  if (i->lo > MAX_DIV_2) {
    i->hi <<= 1;
    i->hi |= 1ULL;
    i->lo <<= 1;
    i->lo |= 1ULL;
    add(i, hi, lo);
  } else {
    i->hi <<= 1;
    i->lo <<= 1;
    i->lo |= 1ULL;
    add(i, hi, lo);
  }
}

int gt(u128_t *a, u128_t *b) {
  if (a->hi < b->hi) {
    return 0;
  } else if (a->hi > b->hi) {
    return 1;
  } else if (a->lo <= b->lo) {
    return 0;
  }
  return 1;
}

int main(int argc, char **argv) {
  if (argc < 2) {
    goto usage;
  }
  u_quad_t upper = strtouq(argv[1], NULL, 10);
  //printf("upper=%llu\n", upper);
  u_quad_t i = 2ULL;

  u128_t j;
  while (i <= upper) {
    //printf("i=%llu\n", i);
    j.hi = 0ULL;
    j.lo = i;
    unsigned int ct = 1;
    while (j.hi > 0ULL || j.lo >= i) {
      //printf("j=(%llu, %llu)\n", j.hi, j.lo);
      ct++;
      if ((j.lo & 1ULL) == 0ULL) {
        div2(&j);
      } else {
        mul3plus1(&j);
      }
      if (gt(&j, &mj)) {
        mj = j;
      }
    }
    if (ct > mct) {
      mct = ct;
      mi = i;
    }
    i++;
  }
  printf("Max i=%llu ct=%u j=(%llu, %llu)\n", mi, mct, mj.hi, mj.lo);
  exit(0);
 usage:
  printf("Usage: collatz <N>\n");
  exit(1);
}

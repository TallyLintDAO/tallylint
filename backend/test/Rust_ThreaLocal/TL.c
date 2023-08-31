#include <stdio.h>
#include <pthread.h>

// Declare a thread-local variable
__thread int counter = 0;

void *increment(void *arg)
{
  // Each thread will see a different value for counter
  printf("Thread %ld: counter = %d\n", (long)arg, counter);
  counter++;
  printf("Thread %ld: counter = %d\n", (long)arg, counter);
  return NULL;
}

int main(void)
{
  pthread_t t1, t2;
  // Create two threads
  pthread_create(&t1, NULL, increment, (void *)1);
  pthread_create(&t2, NULL, increment, (void *)2);
  // Wait for the threads to finish
  pthread_join(t1, NULL);
  pthread_join(t2, NULL);
  return 0;
}

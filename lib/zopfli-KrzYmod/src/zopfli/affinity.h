/*
From: http://yyshen.github.io/2015/01/18/binding_threads_to_cores_osx.html
*/

#ifndef AFFINITY_H_
#define AFFINITY_H_

#ifdef __MACH__

#include <stdint.h>
#include <pthread.h>
#include <mach/thread_policy.h>
#include <mach/thread_act.h>

typedef struct cpu_set {
  uint32_t count;
} cpu_set_t;

static void CPU_ZERO(cpu_set_t* cpu_set) {
  cpu_set->count = 0;
}

static void CPU_SET(uint32_t num, cpu_set_t* cpu_set) {
  cpu_set->count |= (1 << num);
}

static int CPU_ISSET(uint32_t num, cpu_set_t* cpu_set) {
  return (cpu_set->count & (1 << num));
}

static int pthread_setaffinity_np(pthread_t thread, size_t cpu_size, cpu_set_t* cpu_set)
{
  thread_port_t mach_thread;
  uint32_t core = 0;

  for (core = 0; core < 8 * cpu_size; ++core) {
    if (CPU_ISSET(core, cpu_set)) break;
  }
  thread_affinity_policy_data_t policy = {core};
  mach_thread = pthread_mach_thread_np(thread);
  thread_policy_set(mach_thread, THREAD_AFFINITY_POLICY, (thread_policy_t)&policy, 1);
  return 0;
}

#endif /* __MACH__ */

#endif /* AFFINITY_H_ */

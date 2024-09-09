//! NICE_TO_PRIO: nice + DEFAULT_PRIO
//! PRIO_TO_NICE: prio - DEFAULT_PRIO
//! Priority of a process goes from 0..MAX_PRIO-1, valid RT
//! priority is 0..MAX_RT_PRIO-1, and SCHED_NORMAL/SCHED_BATCH
//! tasks are in the range MAX_RT_PRIO..MAX_PRIO-1. Priority
//! values are inverted: lower p->prio value means higher priority.
//! prio越低，优先级越高 

// the range of nice value: -20 ~ 19
const MAX_NICE: i32 = 19;
const MIN_NICE: i32 = -20;
const NICE_WIDTH: i32 = MAX_NICE - MIN_NICE + 1;


const MAX_RT_PRIO: i32 = 100;

const MAX_PRIO: i32 = MAX_RT_PRIO + NICE_WIDTH;
const DEFAULT_PRIO: i32 = MAX_RT_PRIO + NICE_WIDTH / 2;

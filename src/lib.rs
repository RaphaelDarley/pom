pub struct Pom<V> {
    inner: Vec<RunBuffer<V>>,
    // TODO, add opaque phantom type for safety
}

impl<V> Pom<V> {
    pub fn new(start_val: V) -> Pom<V> {
        Pom {
            inner: vec![RunBuffer {
                pred: PomKey::START,
                inner: vec![start_val],
            }],
        }
    }

    pub fn insert(&mut self, pred: PomKey, val: V) -> PomKey {
        let buff = &mut self.inner[pred.buffer_index];

        if buff.inner.len() == pred.inner_index + 1 {
            // The predecessor is that last in the buffer, so we can just push to the end
            buff.inner.push(val);
            PomKey {
                buffer_index: pred.buffer_index,
                inner_index: pred.inner_index + 1,
            }
        } else {
            // We need to start a new run buffer
            self.inner.push(RunBuffer {
                pred,
                inner: vec![val],
            });

            PomKey {
                buffer_index: self.inner.len() - 1,
                inner_index: 0,
            }
        }
    }

    pub fn get(&self, key: PomKey) -> &V {
        let buff = &self.inner[key.buffer_index as usize];
        &buff.inner[key.inner_index as usize]
    }

    pub fn predecessors<'a>(&'a self, key: PomKey) -> PomPredIter<'a, V> {
        PomPredIter {
            pom: self,
            cur_buff: key.buffer_index,
            cur_inner: key.inner_index,
        }
    }
}

pub struct PomPredIter<'a, V> {
    pom: &'a Pom<V>,
    cur_buff: usize,
    cur_inner: usize,
}

impl<'a, V> Iterator for PomPredIter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_buff == 0 && self.cur_inner == 0 {
            return None;
        }

        let out = self.pom.get(PomKey {
            buffer_index: self.cur_buff,
            inner_index: self.cur_inner,
        });

        if self.cur_inner == 0 {
            let buff = &self.pom.inner[self.cur_buff];
            self.cur_buff = buff.pred.buffer_index;
            self.cur_inner = buff.pred.inner_index;
        } else {
            self.cur_inner -= 1;
        }

        Some(out)
    }
}

struct RunBuffer<V> {
    pred: PomKey,
    inner: Vec<V>,
}

impl<V> RunBuffer<V> {}

pub struct PomKey {
    buffer_index: usize,
    inner_index: usize,
}

impl PomKey {
    const START: PomKey = PomKey {
        buffer_index: 0,
        inner_index: 0,
    };
}

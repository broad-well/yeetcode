use std::collections::VecDeque;

trait Stack {
    fn new() -> Self;
    fn push(&mut self, x: i32);
    fn pop(&mut self) -> i32;
    fn top(&mut self) -> i32;
    fn empty(&self) -> bool;
    fn push_pop_ops(&self) -> (usize, usize);
}

struct MyStack {
    queue_a: VecDeque<i32>,
    queue_b: VecDeque<i32>,
    push_ops: usize,
    pop_ops: usize,
    using_a: bool
}

impl Stack for MyStack {
    
    fn new() -> Self {
        Self { queue_a: VecDeque::new(), queue_b: VecDeque::new(), using_a: true, push_ops: 0, pop_ops: 0 }
    }
    
    fn push(&mut self, x: i32) {
        // clear the inactive queue (it should hold at most 1 element)
        if let Some(elem) = self.inactive_queue().pop_front() {
            self.pop_ops += 1;
            self.active_queue().push_back(elem);
            self.push_ops += 1;
        }
        self.inactive_queue().push_back(x);
        self.push_ops += 1;
    }
    
    fn pop(&mut self) -> i32 {
        if self.inactive_queue().is_empty() {
            self.shift_elements();
        }
        assert!(self.inactive_queue().len() == 1);
        self.pop_ops += 1;
        self.inactive_queue().pop_front().unwrap()
    }
    
    // mut because internal rearrangement might be necessary to provide this answer
    fn top(&mut self) -> i32 {
        if self.inactive_queue().is_empty() {
            self.shift_elements();
        }
        assert!(self.inactive_queue().len() == 1);
        *self.inactive_queue().front().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.queue_a.is_empty() && self.queue_b.is_empty()
    }

    fn push_pop_ops(&self) -> (usize, usize) {
        (self.push_ops, self.pop_ops)
    }
}

impl MyStack {
    #[inline]
    fn active_queue(&mut self) -> &mut VecDeque<i32> {
        if self.using_a {
            &mut self.queue_a
        } else {
            &mut self.queue_b
        }
    }

    #[inline]
    fn inactive_queue(&mut self) -> &mut VecDeque<i32> {
        if self.using_a {
            &mut self.queue_b
        } else {
            &mut self.queue_a
        }
    }

    fn shift_elements(&mut self) {
        while self.active_queue().len() > 1 {
            self.pop_ops += 1;
            let elem = self.active_queue().pop_front().unwrap();
            self.push_ops += 1;
            self.inactive_queue().push_back(elem);
        }
        // The one remaining item in the active queue is the one to pop
        self.using_a ^= true; // fancy negation
    }
}

struct OneQueueStack {
    queue: VecDeque<i32>,
    push_ops: usize,
    pop_ops: usize,
}

impl Stack for OneQueueStack {
    fn new() -> Self {
        Self { queue: VecDeque::new(), push_ops: 0, pop_ops: 0 }
    }

    fn push(&mut self, item: i32) {
        self.queue.push_back(item); self.push_ops += 1;
        let mut i = 1;
        // rotate size-1
        while i < self.queue.len() {
            let item = self.queue.pop_front().unwrap(); self.pop_ops += 1;
            self.queue.push_back(item); self.push_ops += 1;
            i += 1;
        }
        assert_eq!(*self.queue.front().unwrap(), item);
    }

    fn pop(&mut self) -> i32 {
        self.pop_ops += 1;
        self.queue.pop_front().unwrap()
    }

    fn top(&mut self) -> i32 {
        *self.queue.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn push_pop_ops(&self) -> (usize, usize) {
        (self.push_ops, self.pop_ops)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn benchmark<T: Stack>() -> (usize, usize) {
        let mut stack = T::new();
        stack.push(84206264);
assert_eq!(stack.pop(), 84206264);
stack.push(2012034);
stack.push(61384876);
assert_eq!(stack.pop(), 61384876);
stack.push(773351);
assert_eq!(stack.pop(), 773351);
assert_eq!(stack.top(), 2012034);
assert_eq!(stack.pop(), 2012034);
stack.push(93070463);
assert_eq!(stack.top(), 93070463);
assert_eq!(stack.pop(), 93070463);
stack.push(24350591);
assert_eq!(stack.top(), 24350591);
stack.push(11332700);
stack.push(50947717);
assert_eq!(stack.top(), 50947717);
assert_eq!(stack.top(), 50947717);
assert_eq!(stack.pop(), 50947717);
assert_eq!(stack.pop(), 11332700);
stack.push(93456325);
assert_eq!(stack.pop(), 93456325);
stack.push(74231535);
assert_eq!(stack.top(), 74231535);
assert_eq!(stack.pop(), 74231535);
assert_eq!(stack.top(), 24350591);
assert_eq!(stack.pop(), 24350591);
stack.push(10181694);
assert_eq!(stack.pop(), 10181694);
stack.push(28640497);
assert_eq!(stack.top(), 28640497);
assert_eq!(stack.pop(), 28640497);
stack.push(50710856);
stack.push(11577945);
stack.push(44261357);
assert_eq!(stack.top(), 44261357);
assert_eq!(stack.top(), 44261357);
assert_eq!(stack.pop(), 44261357);
assert_eq!(stack.pop(), 11577945);
assert_eq!(stack.top(), 50710856);
assert_eq!(stack.top(), 50710856);
stack.push(74914995);
assert_eq!(stack.pop(), 74914995);
assert_eq!(stack.pop(), 50710856);
stack.push(78288601);
assert_eq!(stack.pop(), 78288601);
stack.push(9920273);
assert_eq!(stack.top(), 9920273);
stack.push(28888638);
assert_eq!(stack.top(), 28888638);
stack.push(30263588);
assert_eq!(stack.pop(), 30263588);
assert_eq!(stack.top(), 28888638);
stack.push(87675794);
assert_eq!(stack.top(), 87675794);
stack.push(39050988);
assert_eq!(stack.top(), 39050988);
assert_eq!(stack.top(), 39050988);
assert_eq!(stack.top(), 39050988);
assert_eq!(stack.pop(), 39050988);
assert_eq!(stack.top(), 87675794);
stack.push(83671437);
assert_eq!(stack.pop(), 83671437);
assert_eq!(stack.top(), 87675794);
assert_eq!(stack.pop(), 87675794);
stack.push(18736404);
assert_eq!(stack.top(), 18736404);
assert_eq!(stack.pop(), 18736404);
assert_eq!(stack.pop(), 28888638);
stack.push(61322566);
assert_eq!(stack.top(), 61322566);
assert_eq!(stack.pop(), 61322566);
assert_eq!(stack.pop(), 9920273);
stack.push(70155752);
assert_eq!(stack.top(), 70155752);
stack.push(62502339);
assert_eq!(stack.pop(), 62502339);
assert_eq!(stack.pop(), 70155752);
stack.push(68972865);
assert_eq!(stack.pop(), 68972865);
stack.push(54631840);
assert_eq!(stack.pop(), 54631840);
stack.push(23414581);
assert_eq!(stack.top(), 23414581);
stack.push(35268649);
assert_eq!(stack.top(), 35268649);
stack.push(37961448);
stack.push(42497851);
assert_eq!(stack.top(), 42497851);
stack.push(23294497);
stack.push(32536534);
assert_eq!(stack.top(), 32536534);
assert_eq!(stack.pop(), 32536534);
assert_eq!(stack.pop(), 23294497);
stack.push(86689075);
stack.push(3558644);
stack.push(34587712);
assert_eq!(stack.pop(), 34587712);
assert_eq!(stack.pop(), 3558644);
stack.push(73665416);
stack.push(80728000);
assert_eq!(stack.top(), 80728000);
stack.push(27366164);
assert_eq!(stack.top(), 27366164);
stack.push(90888030);
stack.push(85650580);
assert_eq!(stack.pop(), 85650580);
stack.push(93358876);
assert_eq!(stack.pop(), 93358876);
assert_eq!(stack.pop(), 90888030);
assert_eq!(stack.pop(), 27366164);
stack.push(94164231);
assert_eq!(stack.top(), 94164231);
stack.push(25443396);
assert_eq!(stack.pop(), 25443396);
stack.push(85745429);
assert_eq!(stack.top(), 85745429);
assert_eq!(stack.top(), 85745429);
stack.push(62933670);
assert_eq!(stack.top(), 62933670);
assert_eq!(stack.top(), 62933670);
stack.push(31394633);
assert_eq!(stack.pop(), 31394633);
assert_eq!(stack.top(), 62933670);
assert_eq!(stack.pop(), 62933670);
stack.push(65866933);
stack.push(52099730);
assert_eq!(stack.pop(), 52099730);
assert_eq!(stack.top(), 65866933);
assert_eq!(stack.pop(), 65866933);
stack.push(88002039);
assert_eq!(stack.top(), 88002039);
assert_eq!(stack.pop(), 88002039);
stack.push(59505023);
stack.push(78409752);
assert_eq!(stack.pop(), 78409752);
stack.push(94267828);
assert_eq!(stack.top(), 94267828);
assert_eq!(stack.top(), 94267828);
stack.push(20666491);
stack.push(79500343);
assert_eq!(stack.top(), 79500343);
assert_eq!(stack.pop(), 79500343);
assert_eq!(stack.pop(), 20666491);
assert_eq!(stack.pop(), 94267828);
assert_eq!(stack.top(), 59505023);
stack.push(49949944);
assert_eq!(stack.top(), 49949944);
assert_eq!(stack.pop(), 49949944);
stack.push(12649681);
assert_eq!(stack.top(), 12649681);
stack.push(59563430);
assert_eq!(stack.pop(), 59563430);
stack.push(29499580);
assert_eq!(stack.top(), 29499580);
stack.push(57838023);
stack.push(12795620);
assert_eq!(stack.top(), 12795620);
stack.push(80059785);
assert_eq!(stack.pop(), 80059785);
assert_eq!(stack.pop(), 12795620);
assert_eq!(stack.top(), 57838023);
assert_eq!(stack.top(), 57838023);
stack.push(89609216);
assert_eq!(stack.top(), 89609216);
stack.push(46456028);
stack.push(67171151);
assert_eq!(stack.pop(), 67171151);
assert_eq!(stack.top(), 46456028);
assert_eq!(stack.pop(), 46456028);
assert_eq!(stack.pop(), 89609216);
assert_eq!(stack.pop(), 57838023);
assert_eq!(stack.pop(), 29499580);
stack.push(96344401);
assert_eq!(stack.top(), 96344401);
assert_eq!(stack.pop(), 96344401);
assert_eq!(stack.pop(), 12649681);
assert_eq!(stack.pop(), 59505023);
assert_eq!(stack.pop(), 85745429);
stack.push(99484964);
assert_eq!(stack.pop(), 99484964);
assert_eq!(stack.pop(), 94164231);
assert_eq!(stack.pop(), 80728000);
assert_eq!(stack.pop(), 73665416);
assert_eq!(stack.pop(), 86689075);
stack.push(22608541);
assert_eq!(stack.top(), 22608541);
assert_eq!(stack.pop(), 22608541);
assert_eq!(stack.pop(), 42497851);
assert_eq!(stack.top(), 37961448);
assert_eq!(stack.pop(), 37961448);
assert_eq!(stack.pop(), 35268649);
stack.push(59139898);
stack.push(90814666);
assert_eq!(stack.pop(), 90814666);
assert_eq!(stack.top(), 59139898);
stack.push(11879242);
stack.push(99289312);
assert_eq!(stack.top(), 99289312);
assert_eq!(stack.top(), 99289312);
assert_eq!(stack.top(), 99289312);
stack.push(10931780);
assert_eq!(stack.top(), 10931780);
stack.push(20090530);
assert_eq!(stack.pop(), 20090530);
stack.push(79311014);
assert_eq!(stack.top(), 79311014);
assert_eq!(stack.top(), 79311014);
assert_eq!(stack.pop(), 79311014);
assert_eq!(stack.top(), 10931780);
stack.push(77099089);
assert_eq!(stack.pop(), 77099089);
assert_eq!(stack.top(), 10931780);
stack.push(49278238);
assert_eq!(stack.top(), 49278238);
assert_eq!(stack.top(), 49278238);
assert_eq!(stack.top(), 49278238);
assert_eq!(stack.top(), 49278238);
assert_eq!(stack.top(), 49278238);
assert_eq!(stack.top(), 49278238);
assert_eq!(stack.pop(), 49278238);
assert_eq!(stack.pop(), 10931780);
assert_eq!(stack.top(), 99289312);
assert_eq!(stack.top(), 99289312);
assert_eq!(stack.pop(), 99289312);
stack.push(75586390);
stack.push(96865744);
stack.push(19909536);
assert_eq!(stack.top(), 19909536);
assert_eq!(stack.top(), 19909536);
assert_eq!(stack.top(), 19909536);
stack.push(14111607);
stack.push(82577205);
assert_eq!(stack.pop(), 82577205);
assert_eq!(stack.pop(), 14111607);
stack.push(13856362);
assert_eq!(stack.top(), 13856362);
assert_eq!(stack.top(), 13856362);
stack.push(3555867);
assert_eq!(stack.top(), 3555867);
assert_eq!(stack.pop(), 3555867);
assert_eq!(stack.top(), 13856362);
stack.push(35782345);
assert_eq!(stack.pop(), 35782345);
assert_eq!(stack.top(), 13856362);
assert_eq!(stack.pop(), 13856362);
assert_eq!(stack.top(), 19909536);
stack.push(39190428);
assert_eq!(stack.top(), 39190428);
stack.push(55339345);
assert_eq!(stack.top(), 55339345);
assert_eq!(stack.pop(), 55339345);
assert_eq!(stack.top(), 39190428);
stack.push(60998425);
assert_eq!(stack.top(), 60998425);
assert_eq!(stack.top(), 60998425);
stack.push(53726422);
assert_eq!(stack.pop(), 53726422);
assert_eq!(stack.pop(), 60998425);
assert_eq!(stack.pop(), 39190428);
assert_eq!(stack.pop(), 19909536);
assert_eq!(stack.pop(), 96865744);
assert_eq!(stack.top(), 75586390);
assert_eq!(stack.top(), 75586390);
assert_eq!(stack.pop(), 75586390);
assert_eq!(stack.pop(), 11879242);
stack.push(85164938);
assert_eq!(stack.top(), 85164938);
assert_eq!(stack.pop(), 85164938);
assert_eq!(stack.pop(), 59139898);
assert_eq!(stack.pop(), 23414581);
stack.push(90808464);
stack.push(87772388);
stack.push(31296275);
assert_eq!(stack.pop(), 31296275);
assert_eq!(stack.top(), 87772388);
assert_eq!(stack.pop(), 87772388);
assert_eq!(stack.top(), 90808464);
stack.push(46698851);
assert_eq!(stack.pop(), 46698851);
assert_eq!(stack.top(), 90808464);
assert_eq!(stack.top(), 90808464);
assert_eq!(stack.pop(), 90808464);
stack.push(8608876);
assert_eq!(stack.pop(), 8608876);
stack.push(82109677);
assert_eq!(stack.pop(), 82109677);
stack.push(48208342);
stack.push(36295931);
assert_eq!(stack.pop(), 36295931);
stack.push(59711953);
assert_eq!(stack.pop(), 59711953);
stack.push(6327868);
stack.push(28241659);
assert_eq!(stack.top(), 28241659);
assert_eq!(stack.top(), 28241659);
stack.push(3757991);
assert_eq!(stack.pop(), 3757991);
assert_eq!(stack.top(), 28241659);
stack.push(6056233);
assert_eq!(stack.top(), 6056233);
stack.push(64028134);
stack.push(89436688);
assert_eq!(stack.top(), 89436688);
assert_eq!(stack.top(), 89436688);
assert_eq!(stack.top(), 89436688);
assert_eq!(stack.top(), 89436688);
stack.push(81786141);
assert_eq!(stack.pop(), 81786141);
assert_eq!(stack.top(), 89436688);
stack.push(58723672);
assert_eq!(stack.top(), 58723672);
assert_eq!(stack.top(), 58723672);
assert_eq!(stack.pop(), 58723672);
assert_eq!(stack.top(), 89436688);
assert_eq!(stack.pop(), 89436688);
stack.push(94946509);
assert_eq!(stack.top(), 94946509);
assert_eq!(stack.top(), 94946509);
assert_eq!(stack.pop(), 94946509);
assert_eq!(stack.top(), 64028134);
assert_eq!(stack.top(), 64028134);
stack.push(63622116);
assert_eq!(stack.top(), 63622116);
assert_eq!(stack.top(), 63622116);
stack.push(82571772);
assert_eq!(stack.top(), 82571772);
assert_eq!(stack.pop(), 82571772);
stack.push(32427821);
assert_eq!(stack.pop(), 32427821);
stack.push(5626766);
stack.push(41104000);
assert_eq!(stack.top(), 41104000);
stack.push(41061095);
assert_eq!(stack.top(), 41061095);
stack.push(93571638);
assert_eq!(stack.pop(), 93571638);
stack.push(64991869);
assert_eq!(stack.pop(), 64991869);
assert_eq!(stack.pop(), 41061095);
assert_eq!(stack.pop(), 41104000);
stack.push(94293881);
stack.push(83394262);
assert_eq!(stack.top(), 83394262);
assert_eq!(stack.top(), 83394262);
stack.push(46639664);
assert_eq!(stack.top(), 46639664);
stack.push(64169221);
stack.push(96023727);
assert_eq!(stack.pop(), 96023727);
stack.push(30214486);
assert_eq!(stack.top(), 30214486);
stack.push(41809915);
assert_eq!(stack.top(), 41809915);
assert_eq!(stack.top(), 41809915);
assert_eq!(stack.top(), 41809915);
stack.push(74599050);
assert_eq!(stack.top(), 74599050);
assert_eq!(stack.top(), 74599050);
assert_eq!(stack.pop(), 74599050);
assert_eq!(stack.top(), 41809915);
stack.push(31644749);
assert_eq!(stack.pop(), 31644749);
stack.push(15883283);
assert_eq!(stack.pop(), 15883283);
stack.push(95405124);
assert_eq!(stack.pop(), 95405124);
assert_eq!(stack.top(), 41809915);
stack.push(10992754);
assert_eq!(stack.pop(), 10992754);
assert_eq!(stack.pop(), 41809915);
stack.push(21024492);
assert_eq!(stack.top(), 21024492);
assert_eq!(stack.pop(), 21024492);
stack.push(66680019);
stack.push(34868517);
assert_eq!(stack.top(), 34868517);
assert_eq!(stack.pop(), 34868517);
assert_eq!(stack.pop(), 66680019);
assert_eq!(stack.top(), 30214486);
assert_eq!(stack.top(), 30214486);
stack.push(45722760);
assert_eq!(stack.pop(), 45722760);
assert_eq!(stack.pop(), 30214486);
stack.push(33863427);
assert_eq!(stack.top(), 33863427);
assert_eq!(stack.top(), 33863427);
assert_eq!(stack.pop(), 33863427);
assert_eq!(stack.pop(), 64169221);
stack.push(41578351);
stack.push(36223569);
assert_eq!(stack.pop(), 36223569);
stack.push(99504489);
assert_eq!(stack.top(), 99504489);
stack.push(21827631);
assert_eq!(stack.pop(), 21827631);
assert_eq!(stack.pop(), 99504489);
stack.push(80010820);
assert_eq!(stack.top(), 80010820);
assert_eq!(stack.top(), 80010820);
assert_eq!(stack.top(), 80010820);
stack.push(43078755);
stack.push(15616277);
stack.push(26615102);
stack.push(8379167);
stack.push(83233726);
assert_eq!(stack.pop(), 83233726);
assert_eq!(stack.top(), 8379167);
assert_eq!(stack.top(), 8379167);
assert_eq!(stack.pop(), 8379167);
assert_eq!(stack.pop(), 26615102);
stack.push(82454924);
assert_eq!(stack.pop(), 82454924);
stack.push(90232560);
assert_eq!(stack.top(), 90232560);
assert_eq!(stack.pop(), 90232560);
assert_eq!(stack.top(), 15616277);
assert_eq!(stack.pop(), 15616277);
assert_eq!(stack.pop(), 43078755);
assert_eq!(stack.top(), 80010820);
stack.push(92983167);
assert_eq!(stack.top(), 92983167);
assert_eq!(stack.top(), 92983167);
stack.push(51130812);
assert_eq!(stack.pop(), 51130812);
assert_eq!(stack.top(), 92983167);
stack.push(90172376);
stack.push(64068577);
assert_eq!(stack.pop(), 64068577);
stack.push(52827989);
assert_eq!(stack.top(), 52827989);
assert_eq!(stack.pop(), 52827989);
assert_eq!(stack.top(), 90172376);
stack.push(53774628);
assert_eq!(stack.top(), 53774628);
assert_eq!(stack.top(), 53774628);
assert_eq!(stack.pop(), 53774628);
stack.push(15501273);
assert_eq!(stack.pop(), 15501273);
stack.push(65554655);
assert_eq!(stack.top(), 65554655);
assert_eq!(stack.pop(), 65554655);
assert_eq!(stack.top(), 90172376);
assert_eq!(stack.pop(), 90172376);
stack.push(50644386);
stack.push(49659138);
stack.push(55425445);
assert_eq!(stack.top(), 55425445);
stack.push(85758506);
assert_eq!(stack.top(), 85758506);
assert_eq!(stack.pop(), 85758506);
assert_eq!(stack.top(), 55425445);
assert_eq!(stack.top(), 55425445);
assert_eq!(stack.pop(), 55425445);
assert_eq!(stack.top(), 49659138);
stack.push(62058233);
assert_eq!(stack.pop(), 62058233);
stack.push(50802008);
assert_eq!(stack.top(), 50802008);
assert_eq!(stack.top(), 50802008);
stack.push(15358107);
assert_eq!(stack.pop(), 15358107);
assert_eq!(stack.pop(), 50802008);
assert_eq!(stack.top(), 49659138);
assert_eq!(stack.top(), 49659138);
assert_eq!(stack.pop(), 49659138);
stack.push(43117127);
assert_eq!(stack.top(), 43117127);
stack.push(58547030);
assert_eq!(stack.top(), 58547030);
assert_eq!(stack.top(), 58547030);
stack.push(65880270);
stack.push(62838420);
assert_eq!(stack.pop(), 62838420);
stack.push(36954410);
stack.push(54441633);
assert_eq!(stack.top(), 54441633);
assert_eq!(stack.pop(), 54441633);
assert_eq!(stack.top(), 36954410);
assert_eq!(stack.pop(), 36954410);
assert_eq!(stack.pop(), 65880270);
stack.push(24458393);
assert_eq!(stack.top(), 24458393);
assert_eq!(stack.pop(), 24458393);
assert_eq!(stack.pop(), 58547030);
assert_eq!(stack.pop(), 43117127);
stack.push(70170589);
stack.push(56658767);
assert_eq!(stack.pop(), 56658767);
stack.push(23416503);
assert_eq!(stack.top(), 23416503);
assert_eq!(stack.pop(), 23416503);
stack.push(98410548);
assert_eq!(stack.top(), 98410548);
stack.push(75623766);
assert_eq!(stack.top(), 75623766);
stack.push(73150401);
assert_eq!(stack.top(), 73150401);
stack.push(18963275);
assert_eq!(stack.pop(), 18963275);
assert_eq!(stack.top(), 73150401);
assert_eq!(stack.pop(), 73150401);
assert_eq!(stack.top(), 75623766);
assert_eq!(stack.top(), 75623766);
stack.push(82705091);
assert_eq!(stack.pop(), 82705091);
assert_eq!(stack.top(), 75623766);
assert_eq!(stack.top(), 75623766);
stack.push(71690604);
assert_eq!(stack.top(), 71690604);
stack.push(90870411);
stack.push(76648795);
assert_eq!(stack.pop(), 76648795);
assert_eq!(stack.pop(), 90870411);
assert_eq!(stack.top(), 71690604);
assert_eq!(stack.top(), 71690604);
stack.push(26405290);
assert_eq!(stack.top(), 26405290);
stack.push(64570472);
assert_eq!(stack.pop(), 64570472);
assert_eq!(stack.top(), 26405290);
assert_eq!(stack.pop(), 26405290);
assert_eq!(stack.top(), 71690604);
stack.push(6377792);
assert_eq!(stack.pop(), 6377792);
assert_eq!(stack.top(), 71690604);
assert_eq!(stack.top(), 71690604);
assert_eq!(stack.pop(), 71690604);
assert_eq!(stack.pop(), 75623766);
stack.push(5589280);
assert_eq!(stack.pop(), 5589280);
stack.push(79973667);
stack.push(84995128);
stack.push(62716791);
assert_eq!(stack.pop(), 62716791);
stack.push(2889410);
assert_eq!(stack.top(), 2889410);
stack.push(23780489);
stack.push(43017254);
assert_eq!(stack.top(), 43017254);
stack.push(43890380);
assert_eq!(stack.pop(), 43890380);
assert_eq!(stack.top(), 43017254);
assert_eq!(stack.pop(), 43017254);
assert_eq!(stack.pop(), 23780489);
assert_eq!(stack.pop(), 2889410);
stack.push(73434075);
stack.push(13658383);
assert_eq!(stack.top(), 13658383);
assert_eq!(stack.top(), 13658383);
stack.push(80412264);
assert_eq!(stack.top(), 80412264);
assert_eq!(stack.top(), 80412264);
stack.push(83046338);
assert_eq!(stack.top(), 83046338);
assert_eq!(stack.pop(), 83046338);
assert_eq!(stack.pop(), 80412264);
assert_eq!(stack.top(), 13658383);
assert_eq!(stack.pop(), 13658383);
assert_eq!(stack.pop(), 73434075);
stack.push(73877323);
assert_eq!(stack.pop(), 73877323);
stack.push(80857286);
assert_eq!(stack.pop(), 80857286);
assert_eq!(stack.top(), 84995128);
assert_eq!(stack.pop(), 84995128);
assert_eq!(stack.top(), 79973667);
assert_eq!(stack.top(), 79973667);
assert_eq!(stack.top(), 79973667);
assert_eq!(stack.pop(), 79973667);
assert_eq!(stack.pop(), 98410548);
assert_eq!(stack.top(), 70170589);
assert_eq!(stack.top(), 70170589);
assert_eq!(stack.top(), 70170589);
assert_eq!(stack.pop(), 70170589);
stack.push(58576266);
assert_eq!(stack.top(), 58576266);
stack.push(3656391);
stack.push(40201044);
assert_eq!(stack.top(), 40201044);
stack.push(42173257);
stack.push(53465994);
stack.push(46791069);
assert_eq!(stack.top(), 46791069);
assert_eq!(stack.pop(), 46791069);
assert_eq!(stack.pop(), 53465994);
assert_eq!(stack.pop(), 42173257);
assert_eq!(stack.top(), 40201044);
assert_eq!(stack.top(), 40201044);
stack.push(98360936);
assert_eq!(stack.top(), 98360936);
assert_eq!(stack.pop(), 98360936);
stack.push(69956215);
assert_eq!(stack.pop(), 69956215);
assert_eq!(stack.top(), 40201044);
assert_eq!(stack.pop(), 40201044);
assert_eq!(stack.pop(), 3656391);
assert_eq!(stack.top(), 58576266);
assert_eq!(stack.top(), 58576266);
assert_eq!(stack.pop(), 58576266);
assert_eq!(stack.pop(), 50644386);
stack.push(84754379);
assert_eq!(stack.top(), 84754379);
stack.push(10055340);
stack.push(53427162);
assert_eq!(stack.pop(), 53427162);
assert_eq!(stack.top(), 10055340);
stack.push(74450677);
stack.push(68625948);
assert_eq!(stack.pop(), 68625948);
stack.push(32623665);
stack.push(12281873);
stack.push(10649860);
stack.push(10559195);
assert_eq!(stack.pop(), 10559195);
stack.push(677807);
assert_eq!(stack.pop(), 677807);
assert_eq!(stack.pop(), 10649860);
assert_eq!(stack.pop(), 12281873);
assert_eq!(stack.pop(), 32623665);
assert_eq!(stack.pop(), 74450677);
stack.push(87069583);
assert_eq!(stack.pop(), 87069583);
assert_eq!(stack.pop(), 10055340);
assert_eq!(stack.pop(), 84754379);
stack.push(69953659);
assert_eq!(stack.pop(), 69953659);
assert_eq!(stack.top(), 92983167);
assert_eq!(stack.top(), 92983167);
assert_eq!(stack.pop(), 92983167);
stack.push(55273199);
assert_eq!(stack.top(), 55273199);
stack.push(48121833);
assert_eq!(stack.top(), 48121833);
assert_eq!(stack.top(), 48121833);
assert_eq!(stack.top(), 48121833);
stack.push(17471512);
stack.push(68364698);
assert_eq!(stack.pop(), 68364698);
assert_eq!(stack.pop(), 17471512);
stack.push(69839937);
stack.push(58540316);
stack.push(9206691);
assert_eq!(stack.top(), 9206691);
assert_eq!(stack.top(), 9206691);
stack.push(52301112);
assert_eq!(stack.top(), 52301112);
assert_eq!(stack.top(), 52301112);
assert_eq!(stack.pop(), 52301112);
assert_eq!(stack.pop(), 9206691);
assert_eq!(stack.pop(), 58540316);
assert_eq!(stack.top(), 69839937);
stack.push(97363852);
assert_eq!(stack.pop(), 97363852);
assert_eq!(stack.top(), 69839937);
stack.push(70006035);
assert_eq!(stack.top(), 70006035);
assert_eq!(stack.top(), 70006035);
stack.push(75845997);
stack.push(10622673);
stack.push(38695258);
assert_eq!(stack.pop(), 38695258);
assert_eq!(stack.top(), 10622673);
stack.push(97881195);
assert_eq!(stack.pop(), 97881195);
assert_eq!(stack.top(), 10622673);
stack.push(1538474);
assert_eq!(stack.pop(), 1538474);
assert_eq!(stack.pop(), 10622673);
assert_eq!(stack.pop(), 75845997);
assert_eq!(stack.pop(), 70006035);
stack.push(8379281);
stack.push(61444799);
assert_eq!(stack.pop(), 61444799);
stack.push(94994911);
assert_eq!(stack.top(), 94994911);
assert_eq!(stack.pop(), 94994911);
assert_eq!(stack.top(), 8379281);
stack.push(17323433);
assert_eq!(stack.top(), 17323433);
stack.push(28034157);
stack.push(41287892);
stack.push(90272344);
assert_eq!(stack.top(), 90272344);
assert_eq!(stack.top(), 90272344);
assert_eq!(stack.pop(), 90272344);
assert_eq!(stack.pop(), 41287892);
stack.push(18010741);
assert_eq!(stack.pop(), 18010741);
stack.push(44683711);
stack.push(76045632);
assert_eq!(stack.top(), 76045632);
assert_eq!(stack.top(), 76045632);
stack.push(5678565);
assert_eq!(stack.pop(), 5678565);
assert_eq!(stack.pop(), 76045632);
stack.push(81650616);
assert_eq!(stack.pop(), 81650616);
assert_eq!(stack.top(), 44683711);
assert_eq!(stack.top(), 44683711);
assert_eq!(stack.pop(), 44683711);
assert_eq!(stack.top(), 28034157);
stack.push(97641133);
assert_eq!(stack.top(), 97641133);
assert_eq!(stack.pop(), 97641133);
stack.push(24110507);
assert_eq!(stack.top(), 24110507);
assert_eq!(stack.pop(), 24110507);
stack.push(23892809);
assert_eq!(stack.top(), 23892809);
stack.push(63636694);
stack.push(90813588);
assert_eq!(stack.top(), 90813588);
assert_eq!(stack.pop(), 90813588);
assert_eq!(stack.top(), 63636694);
assert_eq!(stack.pop(), 63636694);
assert_eq!(stack.pop(), 23892809);
assert_eq!(stack.top(), 28034157);
assert_eq!(stack.pop(), 28034157);
assert_eq!(stack.top(), 17323433);
assert_eq!(stack.pop(), 17323433);
assert_eq!(stack.pop(), 8379281);
assert_eq!(stack.pop(), 69839937);
assert_eq!(stack.pop(), 48121833);
assert_eq!(stack.pop(), 55273199);
assert_eq!(stack.pop(), 80010820);
assert_eq!(stack.top(), 41578351);
assert_eq!(stack.pop(), 41578351);
assert_eq!(stack.pop(), 46639664);
assert_eq!(stack.pop(), 83394262);
assert_eq!(stack.top(), 94293881);
assert_eq!(stack.top(), 94293881);
assert_eq!(stack.top(), 94293881);
assert_eq!(stack.top(), 94293881);
assert_eq!(stack.top(), 94293881);
stack.push(82514091);
stack.push(58487994);
stack.push(22130664);
stack.push(55039560);
assert_eq!(stack.pop(), 55039560);
assert_eq!(stack.pop(), 22130664);
assert_eq!(stack.pop(), 58487994);
assert_eq!(stack.pop(), 82514091);
assert_eq!(stack.top(), 94293881);
assert_eq!(stack.top(), 94293881);
assert_eq!(stack.top(), 94293881);
assert_eq!(stack.pop(), 94293881);
assert_eq!(stack.pop(), 5626766);
assert_eq!(stack.top(), 63622116);
assert_eq!(stack.pop(), 63622116);
assert_eq!(stack.pop(), 64028134);
assert_eq!(stack.top(), 6056233);
stack.push(57391042);
assert_eq!(stack.top(), 57391042);
assert_eq!(stack.pop(), 57391042);
assert_eq!(stack.pop(), 6056233);
stack.push(21027963);
stack.push(22998169);
stack.push(63629021);
assert_eq!(stack.top(), 63629021);
assert_eq!(stack.top(), 63629021);
assert_eq!(stack.pop(), 63629021);
assert_eq!(stack.top(), 22998169);
stack.push(99020655);
assert_eq!(stack.pop(), 99020655);
assert_eq!(stack.top(), 22998169);
assert_eq!(stack.top(), 22998169);
assert_eq!(stack.pop(), 22998169);
assert_eq!(stack.top(), 21027963);
stack.push(50840215);
stack.push(57611060);
stack.push(76420796);
assert_eq!(stack.top(), 76420796);
stack.push(49525144);
assert_eq!(stack.pop(), 49525144);
stack.push(23367248);
assert_eq!(stack.pop(), 23367248);
stack.push(43597309);
assert_eq!(stack.top(), 43597309);
stack.push(19779763);
assert_eq!(stack.top(), 19779763);
stack.push(48471143);
assert_eq!(stack.top(), 48471143);
assert_eq!(stack.top(), 48471143);
assert_eq!(stack.pop(), 48471143);
assert_eq!(stack.top(), 19779763);
assert_eq!(stack.pop(), 19779763);
assert_eq!(stack.pop(), 43597309);
assert_eq!(stack.pop(), 76420796);
assert_eq!(stack.pop(), 57611060);
stack.push(9630660);
assert_eq!(stack.pop(), 9630660);
assert_eq!(stack.top(), 50840215);
assert_eq!(stack.pop(), 50840215);
assert_eq!(stack.top(), 21027963);
assert_eq!(stack.pop(), 21027963);
assert_eq!(stack.top(), 28241659);
assert_eq!(stack.top(), 28241659);
assert_eq!(stack.pop(), 28241659);
assert_eq!(stack.pop(), 6327868);
stack.push(14710490);
assert_eq!(stack.top(), 14710490);
assert_eq!(stack.top(), 14710490);
assert_eq!(stack.top(), 14710490);
assert_eq!(stack.pop(), 14710490);
assert_eq!(stack.top(), 48208342);
assert_eq!(stack.pop(), 48208342);
stack.push(34155756);
assert_eq!(stack.top(), 34155756);
assert_eq!(stack.top(), 34155756);
assert_eq!(stack.top(), 34155756);
stack.push(91412481);
assert_eq!(stack.top(), 91412481);
assert_eq!(stack.top(), 91412481);
assert_eq!(stack.pop(), 91412481);
stack.push(68178420);
assert_eq!(stack.pop(), 68178420);
assert_eq!(stack.pop(), 34155756);
stack.push(35589599);
stack.push(6948830);
stack.push(73838667);
assert_eq!(stack.top(), 73838667);
stack.push(69588765);
assert_eq!(stack.pop(), 69588765);
assert_eq!(stack.top(), 73838667);
assert_eq!(stack.top(), 73838667);
stack.push(94841000);
assert_eq!(stack.pop(), 94841000);
stack.push(66991902);
assert_eq!(stack.pop(), 66991902);
stack.push(59098469);
assert_eq!(stack.top(), 59098469);
assert_eq!(stack.pop(), 59098469);
assert_eq!(stack.top(), 73838667);
assert_eq!(stack.top(), 73838667);
assert_eq!(stack.top(), 73838667);
stack.push(36144138);
assert_eq!(stack.top(), 36144138);
assert_eq!(stack.pop(), 36144138);
stack.push(67442035);
stack.push(85098037);
stack.push(63315555);
assert_eq!(stack.top(), 63315555);
assert_eq!(stack.top(), 63315555);
stack.push(20462151);
assert_eq!(stack.pop(), 20462151);
assert_eq!(stack.top(), 63315555);
assert_eq!(stack.pop(), 63315555);
stack.push(90757395);
assert_eq!(stack.pop(), 90757395);
stack.push(31499305);
assert_eq!(stack.top(), 31499305);
assert_eq!(stack.top(), 31499305);
stack.push(74204166);
assert_eq!(stack.pop(), 74204166);
stack.push(8227085);
stack.push(84028967);
stack.push(15808816);
assert_eq!(stack.top(), 15808816);
assert_eq!(stack.pop(), 15808816);
stack.push(68151233);
stack.push(706047);
assert_eq!(stack.top(), 706047);
assert_eq!(stack.top(), 706047);
assert_eq!(stack.top(), 706047);
assert_eq!(stack.top(), 706047);
assert_eq!(stack.pop(), 706047);
assert_eq!(stack.pop(), 68151233);
assert_eq!(stack.top(), 84028967);
stack.push(38917739);
assert_eq!(stack.pop(), 38917739);
assert_eq!(stack.top(), 84028967);
assert_eq!(stack.pop(), 84028967);
assert_eq!(stack.pop(), 8227085);
stack.push(79702471);
stack.push(4530331);
stack.push(7731347);
assert_eq!(stack.pop(), 7731347);
stack.push(51977579);
assert_eq!(stack.pop(), 51977579);
assert_eq!(stack.top(), 4530331);
assert_eq!(stack.pop(), 4530331);
assert_eq!(stack.pop(), 79702471);
assert_eq!(stack.pop(), 31499305);
assert_eq!(stack.pop(), 85098037);
assert_eq!(stack.pop(), 67442035);
assert_eq!(stack.pop(), 73838667);
stack.push(9658334);
assert_eq!(stack.top(), 9658334);
assert_eq!(stack.pop(), 9658334);
stack.push(59526012);
assert_eq!(stack.top(), 59526012);
stack.push(22597629);
assert_eq!(stack.pop(), 22597629);
stack.push(52729293);
assert_eq!(stack.top(), 52729293);
assert_eq!(stack.top(), 52729293);
assert_eq!(stack.pop(), 52729293);
stack.push(54602724);
assert_eq!(stack.pop(), 54602724);
assert_eq!(stack.top(), 59526012);
stack.push(20684771);
stack.push(74725185);
assert_eq!(stack.top(), 74725185);
stack.push(27901459);
assert_eq!(stack.pop(), 27901459);
assert_eq!(stack.pop(), 74725185);
assert_eq!(stack.top(), 20684771);
stack.push(63108555);
stack.push(50930467);
stack.push(39884576);
assert_eq!(stack.pop(), 39884576);
assert_eq!(stack.pop(), 50930467);
stack.push(69727586);
stack.push(83288335);
assert_eq!(stack.top(), 83288335);
assert_eq!(stack.pop(), 83288335);
assert_eq!(stack.pop(), 69727586);
stack.push(79180050);
assert_eq!(stack.pop(), 79180050);
assert_eq!(stack.pop(), 63108555);
stack.push(41497814);
assert_eq!(stack.pop(), 41497814);
stack.push(25478225);
assert_eq!(stack.top(), 25478225);
stack.push(34754055);
assert_eq!(stack.top(), 34754055);
stack.push(17637741);
assert_eq!(stack.pop(), 17637741);
stack.push(12633166);
assert_eq!(stack.top(), 12633166);
assert_eq!(stack.pop(), 12633166);
assert_eq!(stack.pop(), 34754055);
stack.push(70970763);
assert_eq!(stack.pop(), 70970763);
stack.push(50831097);
stack.push(23201151);
assert_eq!(stack.top(), 23201151);
stack.push(83624011);
assert_eq!(stack.pop(), 83624011);
stack.push(68192814);
assert_eq!(stack.top(), 68192814);
stack.push(97493101);
assert_eq!(stack.pop(), 97493101);
stack.push(31912372);
stack.push(35882506);
assert_eq!(stack.top(), 35882506);
assert_eq!(stack.pop(), 35882506);
stack.push(73304307);
assert_eq!(stack.pop(), 73304307);
assert_eq!(stack.pop(), 31912372);
assert_eq!(stack.pop(), 68192814);
stack.push(93626245);
stack.push(51313568);
assert_eq!(stack.top(), 51313568);
assert_eq!(stack.pop(), 51313568);
stack.push(18966398);
assert_eq!(stack.top(), 18966398);
assert_eq!(stack.pop(), 18966398);
stack.push(54612439);
assert_eq!(stack.top(), 54612439);
assert_eq!(stack.top(), 54612439);
stack.push(85834253);
assert_eq!(stack.pop(), 85834253);
assert_eq!(stack.top(), 54612439);
assert_eq!(stack.pop(), 54612439);
assert_eq!(stack.pop(), 93626245);
stack.push(98520794);
assert_eq!(stack.pop(), 98520794);
stack.push(95703080);
assert_eq!(stack.top(), 95703080);
stack.push(7800509);
assert_eq!(stack.top(), 7800509);
stack.push(29850710);
assert_eq!(stack.pop(), 29850710);
stack.push(56978042);
assert_eq!(stack.pop(), 56978042);
assert_eq!(stack.pop(), 7800509);
stack.push(22170724);
assert_eq!(stack.pop(), 22170724);
stack.push(33241289);
stack.push(35205128);
assert_eq!(stack.pop(), 35205128);
assert_eq!(stack.top(), 33241289);
assert_eq!(stack.pop(), 33241289);
assert_eq!(stack.pop(), 95703080);
stack.push(36363565);
assert_eq!(stack.top(), 36363565);
assert_eq!(stack.top(), 36363565);
assert_eq!(stack.pop(), 36363565);
assert_eq!(stack.pop(), 23201151);
assert_eq!(stack.top(), 50831097);
stack.push(5002036);
assert_eq!(stack.top(), 5002036);
assert_eq!(stack.pop(), 5002036);
stack.push(13680707);
assert_eq!(stack.pop(), 13680707);
assert_eq!(stack.pop(), 50831097);
assert_eq!(stack.top(), 25478225);
stack.push(18117071);

        stack.push_pop_ops()
    }

    #[test]
    fn comparison() {
        println!("TwoQueues (#pushes, #pops): {:?}", benchmark::<MyStack>());
        println!("OneQueue (#pushes, #pops): {:?}", benchmark::<OneQueueStack>());
    }
}
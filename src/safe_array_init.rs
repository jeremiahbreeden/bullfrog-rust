
/*****************************************************************************
--                                                                          --
--    Copyright (C) 2018-Present, Jeremiah Breeden                          --
--                                                                          --
-- This is free software;  you can  redistribute it  and/or modify it under --
-- terms of the  GNU General Public License as published  by the Free Soft- --
-- ware  Foundation;  either version 3,  or (at your option) any later ver- --
-- sion.  This is distributed in the hope that it will be useful, but WITH- --
-- OUT ANY WARRANTY;  without even the  implied warranty of MERCHANTABILITY --
-- or FITNESS FOR A PARTICULAR PURPOSE.                                     --
--                                                                          --
-- As a special exception under Section 7 of GPL version 3, you are granted --
-- additional permissions described in the GCC Runtime Library Exception,   --
-- version 3.1, as published by the Free Software Foundation.               --
--                                                                          --
-- You should have received a copy of the GNU General Public License and    --
-- a copy of the GCC Runtime Library Exception along with this program;     --
-- see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see    --
-- <http://www.gnu.org/licenses/>.                                          --
--                                                                          --
--  As a special exception, if you link this unit or a modified copy of     --
--  this unit with other files to produce an executable, this unit does     --
--  not by itself cause the resulting executable to be covered by the       --
--  GNU General Public License. This exception does not however invalidate  --
--  any other reasons why the executable file might be covered by the       --
--  GNU Public License.                                                     --
--                                                                          --
*****************************************************************************/

use std::mem::MaybeUninit;
use std::sync::atomic::{Ordering,fence};

// This is used to manage dropping only the elements that were
// initialized.  
//
// Invariant: count is never higher than the number of initialized
// elements.
struct Wrapper<T, const N: usize>{
   pub elements : MaybeUninit<[T; N]>,
   pub count    : usize
}

impl<T, const N: usize> Drop for Wrapper<T,{N}> {
   fn drop(&mut self){

      // If self.count is less than N, that means a panic happened
      // before initialization finished.  Only allow dropping 
      // initialized data
      if self.count < N {
         let array_ptr = self.elements.as_ptr() as *mut T;
         for i in 0 .. self.count {
            unsafe{array_ptr.add(i).drop_in_place()};
         }
      }
   }
}

// Create an array from a closure.  Each element is initialized
// from a separate call of the closure
pub fn from_closure<F,T,const N: usize>(mut closure : F) -> [T; N] 
   where F: FnMut() -> T
{
   // Start with a drop safe wrapper
   let mut result = Wrapper::<T,{N}> {
      elements : MaybeUninit::uninit(),
      count    : 0
   };

   // write each element.  Make sure count is updated after the write.  This
   // way, if a panic occurs, only the initialized elements will be dropped
   let array_ptr = result.elements.as_ptr() as *mut T;

   while result.count < N {

      // Try initializing each element
      unsafe{array_ptr.add(result.count).write(closure())};

      // Ensure that the count is incremented after modifying the array
      // using a set of atomic fences so that only fully initialized
      // elemetents will be dropped in the event of a panic!()
      fence(Ordering::Release);
      result.count += 1;
      fence(Ordering::Release);
      
   }

   // Cannot transmute out due to the Drop trait, so use read() instead
   let array_ptr = result.elements.as_ptr();
   return unsafe{array_ptr.read()};
}

#[inline]
pub fn from_default<T,const N: usize>() -> [T; N]
   where T: Default 
{
   return from_closure(||{T::default()});
}

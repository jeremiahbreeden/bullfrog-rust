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

pub struct Vector<Element, const CAPACITY : usize> {
    data : [Element; CAPACITY],
    size : usize
}

use super::safe_array_init::*;

impl <Element, const CAPACITY: usize> Vector<Element,CAPACITY> {

    pub fn new<F>(f : F, length : usize) -> Self
        where F: FnMut() -> Element
    {
        if length > CAPACITY {
            panic!("Vector::new() invalid length value");
        }
        return Self {
            data : from_closure(f),
            size : length
        };
    }

    pub fn new_empty<F>(f : F) -> Self
        where F: FnMut() -> Element
    {
        return Self {
            data : from_closure(f),
            size : 0
        };
    }

    pub fn new_full<F>(f : F) -> Self
        where F: FnMut() -> Element
    {
        return Self {
            data : from_closure(f),
            size : CAPACITY
        };
    }

    pub fn capacity(&self) -> usize{
        return CAPACITY;
    }

    pub fn length(&self) -> usize{
        return self.size;
    }
    
    pub fn append(&mut self, element : Element) -> bool {
        if self.size >= CAPACITY {
            return false;
        }
        self.data[self.size] = element;
        self.size = self.size + 1;
        return true;
    }

    pub fn get(&self, position : usize) -> Option<&Element>{
        if self.size <= position {
            return None;
        }
        return Some(&self.data[position]);
    }

    pub fn get_mut(&mut self, position : usize) -> Option<&mut Element>{
        if self.size <= position {
            return None;
        }
        return Some(&mut self.data[position]);
    }
    pub fn delete_last(&mut self) -> bool {
        if self.size == 0{
            return false;
        }
        self.size = self.size - 1;
        return true;
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        return self.size >= CAPACITY;
    }
}

impl <Element, const CAPACITY: usize> Vector<Element,CAPACITY> 
    where Element : Clone
{
    pub fn insert(&mut self, element : Element) -> bool {
        if self.size >= CAPACITY {
            return false;
        }

        let mut index = self.size;
        while index > 0 {
            self.data[index] = self.data[index - 1].clone();
            index = index - 1;
        }
        self.data[0] = element.clone();
        self.size = self.size + 1;
     
        return true;
    }

    pub fn insert_at
        (&mut self,
         element  : Element,
         position : usize)
        -> bool
    {
        if self.size >= CAPACITY {
            return false;
        }
        if position > self.size {
            return false;
        }

        let mut index = self.size;
        while index > position {
            self.data[index] = self.data[index - 1].clone();
            index = index - 1;
        }
        self.data[position] = element.clone();
        self.size = self.size + 1;
        return true;
    }
    pub fn delete(&mut self, position : usize) -> bool{
        if position >= self.size {
            return false;
        }
        if self.size == 0 {
            return false;
        }

        for index in position .. self.size-1 {
            self.data[index] = self.data[index+1].clone();
        }
        self.size = self.size - 1;
        return true;
    }

    pub fn delete_first(&mut self) -> bool {
        if self.size == 0 {
            return false;
        }
        for index in 0 .. self.size - 1{
            self.data[index] = self.data[index+1].clone();
        }
        self.size = self.size - 1;
        return true;
    }
    pub fn remove(&mut self, position : usize) -> Option<Element>{
        if position >= self.size {
            return None;
        }
        if self.size == 0 {
            return None;
        }
        let result = self.data[position].clone();
        for index in position .. self.size-1 {
            self.data[index] = self.data[index+1].clone();
        }
        self.size = self.size - 1;
        return Some(result);
    }
    pub fn remove_first(&mut self) -> Option<Element>{
        if self.size == 0 {
            return None;
        }
        let result = self.data[0].clone();
        for index in 0 .. self.size - 1{
            self.data[index] = self.data[index+1].clone();
        }
        self.size = self.size - 1;
        return Some(result);
    }
    pub fn remove_last(&mut self)  -> Option<Element>{
        if self.size == 0 {
            return None;
        }
        
        self.size = self.size - 1;
        return Some(self.data[self.size].clone());
    }
}

impl <Element, const CAPACITY: usize> Vector<Element,CAPACITY> 
    where Element : PartialEq
{
    pub fn find_first(&self, element : Element) -> Option<usize> {
        for index in 0..self.size {
            if self.data[index] == element {
                return Some(index);
            }
        }
        return None;
    }

    pub fn find_last(&self, element : Element) -> Option<usize> {
        let mut index = self.size;
        while index > 0 {
            index = index - 1;
            if self.data[index] == element{
                return Some(index);
            }
        }
        return None;
    }

    pub fn find_next
        (&self,
         element  : Element,
         position : usize)
         -> Option<usize>
    {
        for index in position .. self.size {
            if self.data[index] == element {
                return Some(index);
            }
        }
        return None;
    }
    pub fn find_previous
        (&self,
         element  : Element,
         position : usize)
         -> Option<usize>
    {
        let mut index = self.size;
        while index > position {
            index = index - 1;
            if self.data[index] == element{
                return Some(index);
            }
        }
        return None;
    }
}

impl<Element,const CAPACITY: usize> Default for Vector<Element,CAPACITY> 
    where Element : Default
{
    fn default() -> Self {
        return Self{
            data : from_default(),
            size : 0
        };
    }
}

impl<Element,const CAPACITY: usize> Clone for Vector<Element,CAPACITY>
    where Element : Clone
{
    fn clone(&self) -> Self {
        return Self {
            data : self.data.clone(),
            size : self.size
        };
    }
}
impl<Element,const CAPACITY: usize> Copy for Vector<Element,CAPACITY>
    where Element : Copy + Clone
{}

impl<Element,const CAPACITY: usize> From<Element> for Vector<Element,CAPACITY> 
    where Element : Clone
{
    fn from(value : Element) -> Self {
        return Self {
            data : from_closure(||{ return value.clone();}),
            size : CAPACITY
        };
    }
}

impl<Element,const CAPACITY: usize> From<[Element; CAPACITY]> for Vector<Element,CAPACITY>
{
    fn from(value : [Element; CAPACITY]) -> Self {
        return Self {
            data : value,
            size : CAPACITY
        };
    }
}

impl<Element,const CAPACITY: usize> From<&[Element]> for Vector<Element,CAPACITY>
    where Element : Default + Clone
{
    fn from(value : &[Element]) -> Self {
        let mut count = 0;
        let f = ||{
            if count < value.len() {
                count += 1;
                return value[count-1].clone();
            }else{
                return Element::default();
            }
        };
        return Self {
            data : from_closure(f),
            size : value.len()
        };
    }
}

impl<Element,const CAPACITY: usize> PartialEq for Vector<Element,CAPACITY> 
    where Element : PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for index in 0 .. self.size {
            if self.data[index] != other.data[index] {
                return false;
            }
        }
        return true;
    }
}
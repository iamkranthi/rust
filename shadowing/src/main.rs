fn main() {
   let x=10;
   //*inner scope started */
   {
   let x=x+1; 
   //* x value updated*/
   println!("the value of x is {x}");
   }
   
   println!("the value of x is {x}");
}

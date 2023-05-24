# This function prints all the prime numbers
fn main(){
  let primes = [2,3,5,6,7,9,11,13,17,19,23,31];
  for prime in &primes{
    println!("{}", prime);
  }
}

# This function prints only prime numbers which are less than 10 and but executes the entire list
fn main(){
  let primes = [2,3,5,6,7,9,11,13,17,19,23,31];
  for prime in &primes{
    if *prime < 10{
      println!("{}", prime);
    }
  }
}
                   
 # This function print prime numbers less than 10 and breaks
 fn main(){
  let primes = [2,3,5,6,7,9,11,13,17,19,23,31];
  for prime in &primes{
    if *prime > 10{
      break;
      }
    println!("{}", prime);
  }
}

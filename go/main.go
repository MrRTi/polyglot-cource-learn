package main

import "fmt"

func returnsError(value int) error {
  return fmt.Errorf("This is an error with value %v", value)
}

type Foo struct { }

func (f *Foo) thisIsOnFoo(value int) error {
  return fmt.Errorf("This is an error with value %v", value)
}

func CreateFoo(fail bool) (Foo, error) {
  if fail {
    return Foo{}, fmt.Errorf("This is an error")
  }
  return Foo{}, nil
}

func CreateFooWithPointer(fail bool) (*Foo, error) {
  if fail {
    return nil, fmt.Errorf("This is an error")
  }
  return &Foo{}, nil
}


func main() {
  err := returnsError(5) 
  foo := Foo{}

  foo.thisIsOnFoo(6)

  bar,err := CreateFoo(false)
  if err != nil {
    fmt.Printf("oops")
  }
}

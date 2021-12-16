package parser

import (
    "fmt"
    
    . "github.com/ipratt-code/oxyl/std"
)

type Node interface{
    Type() Type
    Print(indent string)
}

type LiteralNode struct {
    val *Value
}

func NewLiteralNode(value *Value) *LiteralNode {
    return &LiteralNode{
        val: value,
    }
}

func (n *LiteralNode) Type() Type {
    return n.val.Type()
}

func (n *LiteralNode) Print(indent string) {
    fmt.Printf("%sliteral node:\n", indent)
    fmt.Printf("%s\ttype: %v\n", indent, n.val.Type().Name())
    fmt.Printf("%s\tvalue: %v\n", indent, n.val.Value())
}


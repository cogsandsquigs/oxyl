package main

import (
  "github.com/gin-gonic/gin"
  "net/http"
)

func APIEngine() *gin.Engine {
  r := gin.Default()

  r.GET("/", hello)

  return r
}

func hello(c *gin.Context) {
  c.JSON(
    http.StatusOK,
    gin.H{
      "hello": "world!",
    },
  )
} 
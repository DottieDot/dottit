package main

import (
	"dottit/comment_service/api"
	"dottit/comment_service/database"
	"dottit/comment_service/model"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/graph-gophers/graphql-go"
	"github.com/graph-gophers/graphql-go/relay"
	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

func main() {
	connectionString := fmt.Sprintf("root:%s@tcp(%s:%s)/%s?parseTime=true", os.Getenv("MYSQL_PASSWORD"), os.Getenv("MYSQL_ADDR"), os.Getenv("MYSQL_PORT"), os.Getenv("MYSQL_DATABASE"))
	db, err := gorm.Open(mysql.Open(connectionString), &gorm.Config{})

	if err != nil {
		panic("Failed to connect to database")
	}

	db.AutoMigrate(&model.Comment{})

	commentRepo := database.NewCommentRepo(*db)

	resolver := api.NewRootResolver(&commentRepo)

	schema := graphql.MustParseSchema(api.Schema, &resolver)
	http.Handle("/query", &relay.Handler{Schema: schema})
	log.Fatal(http.ListenAndServe(":8080", nil))
}

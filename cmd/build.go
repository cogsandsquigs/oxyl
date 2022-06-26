package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

func init() {
	rootCmd.AddCommand(BuildCmd)
}

var BuildCmd = &cobra.Command{
	Use:   "build",
	Short: "Builds the oxyl project in this directory.",
	Long:  `Builds the oxyl project in this directory.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("not available yet, sorry :(")
	},
}

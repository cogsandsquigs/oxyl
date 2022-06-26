package cmd

import (
	"fmt"
	"oxyl/info"

	"github.com/spf13/cobra"
)

func init() {
	rootCmd.AddCommand(versionCmd)
}

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Prints Oxyl's version number",
	Long:  `Prints Oxyl's version number`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("Running Oxyl version %s\n", info.Version)
	},
}

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
	Short: "Prints oxyl's version number",
	Long:  `Prints oxyl's version number`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("%s", info.Version)
	},
}

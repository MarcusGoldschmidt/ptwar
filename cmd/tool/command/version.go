package command

import (
	"github.com/MarcusGoldschmidt/ptwar/pkg"
	"github.com/spf13/cobra"
)

func versionCmd() *cobra.Command {
	return &cobra.Command{
		Use:   "version",
		Short: "Print the version number of flowtool",
		Long:  "All software has versions. This is flowtool's",
		RunE: func(cmd *cobra.Command, args []string) error {
			cmd.Printf("VERSION: %s\n", pkg.Version)
			cmd.Printf("COMMIT: %s\n", pkg.Commit)
			cmd.Printf("BUILD BY: %s\n", pkg.BuiltBy)
			cmd.Printf("BUILD AT: %s\n", pkg.BuiltAt)

			return nil
		},
	}
}

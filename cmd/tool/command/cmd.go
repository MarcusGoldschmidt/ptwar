package command

import (
	"context"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

func Execute(ctx context.Context) error {
	cmd, err := newCommand(ctx)

	if err != nil {
		return err
	}

	err = cmd.Execute()

	return err
}

func newCommand(ctx context.Context) (*cobra.Command, error) {
	cmd := &cobra.Command{
		Use:   "ptwar-tool",
		Short: "Tool for PTWar",
	}

	cmd.SetContext(ctx)

	cmd.AddCommand(versionCmd())
	cmd.AddCommand(showMapCmd())

	err := viper.BindPFlags(cmd.Flags())
	if err != nil {
		return nil, err
	}

	return cmd, nil
}

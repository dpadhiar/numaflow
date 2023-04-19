/*
Copyright 2022 The Numaproj Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

package commands

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

func NewServerInitCommand() *cobra.Command {
	var (
		baseHref string
	)

	command := &cobra.Command{
		Use:   "server-init",
		Short: "Initialize base path for Numaflow server",
		Run: func(cmd *cobra.Command, args []string) {

			reactVar := fmt.Sprintf("REACT_APP_BASE_HREF=%s", baseHref)

			os.WriteFile("/ui/.env", []byte(reactVar), 0666)
		},
	}

	command.Flags().StringVar(&baseHref, "base-href", "/", "Set React environment variable to change basepath of the server.")
	return command
}

package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"github.com/brianvoe/gofakeit/v7"
)

func main() {
	numProjects := flag.Int("count", 5, "Number of test projects to create")
	baseDir := flag.String("base", ".", "Base directory where project directories will be created")
	flag.Parse()

	gofakeit.Seed(time.Now().UnixNano())

	for i := 0; i < *numProjects; i++ {
		createProject(i+1, *baseDir)
	}
}


func createProject(projectNum int, baseDir string) {

	languagesList := []string{"Go", "Python", "JavaScript", "TypeScript", "Rust", "Java", "C#"}
	buildSystemsList := []string{"Make", "CMake", "Gradle", "Maven", "Webpack", "Bazel"}
	idesList := []string{"VSCode", "IntelliJ", "Vim", "Emacs", "Atom", "Sublime"}

	title := gofakeit.AppName()
	sanitizedTitle := sanitizeTitle(title)
	projectDir := filepath.Join(baseDir, sanitizedTitle)

	err := os.MkdirAll(projectDir, 0755)
	if err != nil {
		log.Printf("Project %d: failed to create directory %s: %v", projectNum, projectDir, err)
		return
	}

	var description string
	if gofakeit.Bool() {
		description = gofakeit.Sentence(10)
	}

	numCategories := gofakeit.Number(0, 3)
	categories := make([]string, 0, numCategories)
	for j := 0; j < numCategories; j++ {
		categories = append(categories, gofakeit.Word())
	}

	numLanguages := gofakeit.Number(1, 3)
	languageSet := make(map[string]struct{})
	for len(languageSet) < numLanguages {
		lang := languagesList[gofakeit.Number(0, len(languagesList)-1)]
		var version string
		if gofakeit.Bool() {
			version = fmt.Sprintf("%d.%d", gofakeit.Number(1, 5), gofakeit.Number(0, 9))
		} else {
			version = "unknown"
		}
		key := fmt.Sprintf("%s:%s", lang, version)
		languageSet[key] = struct{}{}
	}

	languages := make([]string, 0, len(languageSet))
	for k := range languageSet {
		languages = append(languages, k)
	}

	numBuildSystems := gofakeit.Number(0, 2)
	buildSystemSet := make(map[string]struct{})
	for len(buildSystemSet) < numBuildSystems {
		bs := buildSystemsList[gofakeit.Number(0, len(buildSystemsList)-1)]
		var version string
		if gofakeit.Bool() {
			version = fmt.Sprintf("%d.%d", gofakeit.Number(1, 3), gofakeit.Number(0, 9))
		} else {
			version = "unknown"
		}
		key := fmt.Sprintf("%s:%s", bs, version)
		buildSystemSet[key] = struct{}{}
	}

	buildSystems := make([]string, 0, len(buildSystemSet))
	for k := range buildSystemSet {
		buildSystems = append(buildSystems, k)
	}

	var preferredIde string
	if gofakeit.Bool() {
		ide := idesList[gofakeit.Number(0, len(idesList)-1)]
		preferredIde = fmt.Sprintf("%s:%d.%d", ide, gofakeit.Number(1, 2), gofakeit.Number(0, 9))
	}

	var repositoryUrl string
	if gofakeit.Bool() {
		repositoryUrl = gofakeit.URL()
	}

	args := []string{"create", "-n"}
	args = append(args, "-d", projectDir)
	args = append(args, "-t", title)

	if len(categories) > 0 {
		args = append(args, "-c")
		args = append(args, categories...)
	}

	if len(languages) > 0 {
		args = append(args, "-l")
		args = append(args, languages...)
	}

	if preferredIde != "" {
		args = append(args, "-p", preferredIde)
	}

	if len(buildSystems) > 0 {
		args = append(args, "-b")
		args = append(args, buildSystems...)
	}

	if description != "" {
		args = append(args, "-D", description)
	}

	if repositoryUrl != "" {
		args = append(args, "-r", repositoryUrl)
	}

	log.Printf("Project %d: Executing: episko_cli %s", projectNum, strings.Join(args, " "))

	cmd := exec.Command("episko_cli", args...)
	output, err := cmd.CombinedOutput()
	if err != nil {
		log.Printf("Project %d (%q): error executing command: %v\nOutput: %s", projectNum, title, err, output)
	} else {
		log.Printf("Project %d (%q) created successfully.", projectNum, title)
	}
}


func sanitizeTitle(title string) string {
	safe := strings.ReplaceAll(title, " ", "_")
	return safe
}

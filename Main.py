import sys
import subprocess

def main():
  res = subprocess.run(["cargo", "run", sys.argv[1]], capture_output=True, text=True)
  print(res.stdout, end='')

if __name__ == "__main__":
  main()
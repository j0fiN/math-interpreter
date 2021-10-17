

def read_file(name):
    try:
        with open(f'{name}', 'r') as f:
            expression = f.read()
    except Exception as e:
        print(e)

def interpret():
    read_file('test.math')

if __name__ == "__main__":
    interpret()
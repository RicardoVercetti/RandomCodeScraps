import random

class BarcodeGenerator:
    def __init__(self):
        self.random = random.Random()

    def generate_ean13_barcode(self):
        base = ''.join(str(self.random.randint(0, 9)) for _ in range(11))
        check_digit = self.calculate_check_digit(base)
        return base + check_digit

    def calculate_check_digit(self, barcode):
        total = 0
        for i, char in enumerate(barcode):
            digit = int(char)
            total += digit if i % 2 == 0 else digit * 3

        mod = total % 10
        return '0' if mod == 0 else str(10 - mod)

    def create_random_barcode(self):
        gen_b = self.generate_ean13_barcode()
        return self.calculate_check_digit(gen_b) + gen_b

# generator = BarcodeGenerator()
# barcode = generator.create_random_barcode()
# print(barcode)

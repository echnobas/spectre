class Monster:
    def __init__(self, name, hp):
        self.name = name
        self.hp = hp

class Friend(Monster):
    def __init__(self, name, hp, high_five_msg, gift):
        super().__init__(name, hp)
        self.high_five_msg = high_five_msg
        self.gift = gift

    def high_five(self):
        print(f"{self.name} highfived you: \"{self.high_five_msg}\" and gave you a gift \"{self.gift}\"")

class Enemy(Monster):
    def __init__(self, name, hp, fight_msg, weakness):
        super().__init__(name, hp)
        self.fight_msg = fight_msg
        self.weakness = weakness

    def challenge(self):
        print(f"{self.name} says: \"{self.fight_msg}\"")
        weapons = ["sword", "dagger"]
        choice = None
        while not choice:
            try:
                choice = int(input("Enter weapon choice (" + ", ".join([f"{i+1} {v}" for i, v in enumerate(weapons)]) + "): "))
                if not (choice >= 1 and choice <= 2):
                    raise ValueError()
            except ValueError:
                print("Invalid choice")
                choice = None
        if weapons[choice-1] == self.weakness:
            print("Player win")
        else:
            print("Player lose")

f = Friend("joe", 1, "well done!!", "dagger")
e = Enemy("joe", 1, "die", "dagger")
f.high_five()
e.challenge()

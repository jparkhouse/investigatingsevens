from random import shuffle, choice

class Card:
    def __init__(self, value, suit):
        if isinstance(value, int):
            if 0 < value < 14:
                self.value = value
            else:
                raise ValueError("Provided value {0} is out of bound 1-13".format(value))
        else:
            raise TypeError("Provided value {0} is not of type int".format(value))
        if isinstance(suit, str):
            if suit in ["a", "b", "c", "d"]:
                self.suit = suit
            else:
                raise ValueError("Provded suit {0} is not valid ('a', 'b', 'c', or 'd')".format(suit))
        else:
            raise TypeError("Provided suit {0} is not of type str".format(suit))
    
    def __str__(self):
        return str(self.value)+self.suit
    
    def __eq__(self, other):
        if isinstance(other, Card):
            if self.value == other.value and self.suit == other.suit:
                return True
            return False
        raise TypeError("== comparison only defined for type Card")

    def __hash__(self) -> int:
        return hash(str(self))

def new_shuffle():
    '''Creates and returns a new deck, with cards shuffled'''
    output = []
    for i in ["a", "b", "c", "d"]:
        for j in range(1, 14):
            output.append(Card(j, i))
    shuffle(output)
    return output

class Game_State_Manager:
    def __init__(self):
        '''A class to manage the game state. Initialises a new game state, with no cards played.'''
        self.game_state = {"a_up": None, "a_down": None, "b_up": None, "b_down": None, "c_up": None, "c_down": None, "d_up": None, "d_down": None}
        self._initial_game_state ={"a_up": None, "a_down": None, "b_up": None, "b_down": None, "c_up": None, "c_down": None, "d_up": None, "d_down": None}
        
    def get_playable_cards(self):
        '''Looks at the current state, and returns all playable cards'''
        if self.game_state == self._initial_game_state:
            return [Card(7, "a")]
        playable = []
        for i in ["a", "b", "c", "d"]:
            if self.game_state[i+"_up"] is None:
                playable.append(Card(7, i))
            else:
                up = self.game_state[i+"_up"]
                down = self.game_state[i+"_down"]
                if up < 13:
                    playable.append(Card(up + 1, i))
                if down > 1:
                    playable.append(Card(down - 1, i))
        if not playable:
            return None
        return playable
        
    def play(self, card):
        '''Takes one Card instance as an argument and either updates the game state or raises an unplayable error'''
        if isinstance(card, Card): # confirm the card is a card, and not some other thing
            if card in self.get_playable_cards(): # check to see if card is playable
                if card.value == 7: # if the card is a seven, handle that case
                    self.game_state[card.suit + "_up"] = 7
                    self.game_state[card.suit + "_down"] = 7
                else:
                    if card.value > 7:
                        self.game_state[card.suit + "_up"] += 1
                    else:
                        self.game_state[card.suit + "_down"] -= 1
            else:
                raise ValueError("Card {0} is currently not playable".format(card))
        else:
            raise TypeError("Provided card {0} is not an instance of Card".format(card))


#any tests


#game setup

players = 6

player_hands = []

for i in range(players):
    player_hands.append([])

deck = new_shuffle()
game = Game_State_Manager()

i = 0
while deck:
    card = deck.pop()
    player_hands[i].append(card)
    i += 1
    if i == players:
        i = 0

decisions = {x + 1: 0 for x in range(players)}
result = {x + 1: 0 for x in range(players)}

#main game loop

current_player = 0
while not ((currently_playable := game.get_playable_cards()) is None):
    if player_hands[current_player]:
        useful_cards = [card for card in currently_playable if card in player_hands[current_player]]
        if len(useful_cards) == 0:
            print("Player {0} knocks".format(current_player + 1))
        elif len(useful_cards) == 1:
            print("Player {0} plays {1}".format(current_player + 1, useful_cards[0]))
            game.play(useful_cards[0])
            player_hands[current_player].pop(player_hands[current_player].index(useful_cards[0]))
        elif len(useful_cards) > 1:
            print("Decision point reached for player {0}".format(current_player + 1))
            decisions[current_player + 1] += 1
            p = choice(useful_cards)
            print("Player {0} decides to play {1}".format(current_player + 1, p))
            game.play(p)
            player_hands[current_player].pop(player_hands[current_player].index(p))
    else:
        result[current_player + 1] += 1
    current_player += 1
    if current_player == players:
        current_player = 0
print(decisions)
result_sort = [(k, v) for k, v in result.items()]
result_sort.sort(key = lambda x: x[1])
print([y[0] for y in result_sort])
"Pygame tryout"
import random
import pygame
from pygame.constants import QUIT

WIDTH = 1280
HEIGHT = 720
# pygame.base.init()

screen = pygame.display.set_mode((WIDTH, HEIGHT))
clock = pygame.time.Clock()
RUNNING = True


# task
# draw a circle that will auto hit the edges and get bounced back in the opposite direction
# requirements:
# 1. [OK] being able to put a circle
# 2. [OK] circle has a width? starting position in the screen/canvas
# 3. [OK] has a momentum to one direction
# 4. [OK] when the edge of the circle touches(equal to or greater than the edges), the momentum reverses
# 5. [OK] this repeats without any external input

# extras
# 1. [OK] spawn in random with random size
# 2. [OK] spawn several other loons and make them move in random
# 3. [ ] when they collide with each other, they should bounce opposite?
# 4. [ ] smaller ones get more momentum when hitting against bigger ones
# 5. [ ] try acceleration part soon after they hit against one another, then return to their constant speeds


class Circle:
    "A single circle you can draw in a pygame screen"
    def __init__(self, x_pos, y_pos, x_dir, y_dir, size):
        self.x_pos = x_pos
        self.y_pos = y_pos
        self.x_dir = x_dir
        self.y_dir = y_dir
        self.size = size

    def update_position(self):
        """update the position of the current circle, direction is 
        changed when the circle reaches any one of the edges"""
        # change direction if we hit the corners
        half_size = self.size
        if (self.x_pos + half_size) >= WIDTH or (self.x_pos - half_size) <= 0:
            self.x_dir *= -1
        if (self.y_pos + half_size) >= HEIGHT or (self.y_pos - half_size) <= 0:
            self.y_dir *= -1

        # movements
        self.x_pos += self.x_dir
        self.y_pos += self.y_dir

    @staticmethod
    def random_pos():
        """Create a random circle parameters with random speed and direction"""
        speed = random.randrange(7, 12)
        x_dir = -(speed) if random.randrange(6) % 2 == 0 else speed
        y_dir = -(speed) if random.randrange(6) % 2 == 0 else speed
        # size = random.randrange(100, 120)
        size = random.randrange(40, 100)
        return Circle(random.randrange(WIDTH), random.randrange(HEIGHT), x_dir, y_dir, size)

player_pos = pygame.Vector2(WIDTH/2, HEIGHT/2)
# last_pressed = time.time()
list_of_circle = [Circle.random_pos() for i in range(10)]

while RUNNING:
    # check for quit event I guess
    for event in pygame.event.get():
        if event.type == QUIT:
            print("pressed the close button!")
            RUNNING = False
            continue        # perhaps return right away than going through the rest of the loop


    # screen filling first
    screen.fill("purple")        # maybe try some other color next time

    for circle in list_of_circle:
        circle.update_position()
        # draw the content
        pygame.draw.circle(screen, "red", pygame.Vector2(circle.x_pos, circle.y_pos), circle.size)

    pygame.display.flip()
    clock.tick(60)

# pylint: disable=no-member
pygame.quit()

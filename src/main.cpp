#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Window/Event.hpp>
#include <SFML/Window/VideoMode.hpp>
#include <iostream>

#include <SFML/Graphics.hpp>

using namespace std;

int main()
{
    cout << "Hello World!" << endl;

    sf::RenderWindow window(sf::VideoMode(200, 200), "Yaah working");
    sf::CircleShape shape(100.f);

    window.clear();
    window.draw(shape);
    window.display();

    return 0;
}

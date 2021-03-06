# Домашнее задание

Делим и тестируем прототип "умного дома"

Цель: Научиться разделять код на модули, использовать сущности из других модулей, писать модульные и функциональные тесты.

Результатом является: Многофайловый прототип библиотеки "умный дом" с тестами.

Разделить логически целостные элементы библиотеки ""умный дом"" на отдельные файлы.

Покрыть тестами требования к библиотеке.

Требования:

* Дом имеет название и содержит несколько помещений.
* Библтотека позволяет запросить список помещений, добавлять и удалять помещения в доме.
* Помещение имеет уникальное название и содержит несколько устройств.
* Устройство имеет уникальное в рамках помещения название, тип и описание.
* Библтотека позволяет добавлять, получать и удалять любое устройство в доме. Получать список устройств в помещении.
* Типы устройств: термометр, умная розетка.
* Термометр позволяет узнать температуру.
* Умная розетка позволяет включать и выключать себя. Предоставляет информацию о текущем состоянии и потребляемой мощности.
* Библиотека позволяет строить отчёт о состоянии всех устройств в доме.

Критерии оценки:

Статус "Принято" ставится, если:

* Проект логически верно разбит на файлы.
* Для каждого пункта требований, найдётся тест, использующий его функционал.
* Тесты компилируются. Успешное выполнение тестов не требуется.
